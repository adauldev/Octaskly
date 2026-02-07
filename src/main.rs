// Main entry point for Octaskly distributed task scheduler
// Titik masuk utama untuk Octaskly penjadwal tugas terdistribusi
//
// Supports: dispatcher mode (task scheduling) and worker mode (task execution)
// Mendukung: mode dispatcher (penjadwalan tugas) dan mode worker (eksekusi tugas)

use anyhow::Result;
use clap::Parser;
use octaskly::cmd::Cli;
use octaskly::scheduler::Scheduler;
use octaskly::state::{DispatcherState, WorkerState};
use octaskly::executor::Executor;
use octaskly::protocol::{Message, WorkerInfo};
use octaskly::util;
use std::path::PathBuf;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::{Duration, interval};
use tokio::sync::RwLock;
use tracing::{error, info, warn, debug};

#[tokio::main]
async fn main() -> Result<()> {
    // Self-installation logic
    // Logika instalasi otomatis
    let install_dir = dirs::home_dir().unwrap().join("bin");
    let install_path = install_dir.join("octaskly.exe");
    let current_exe = std::env::current_exe().unwrap();

    if current_exe != install_path {
        // Install the binary globally
        // Instal binary secara global
        std::fs::create_dir_all(&install_dir).unwrap();

        // Read current binary content into memory
        // Baca konten binary saat ini ke memori
        let binary_content = std::fs::read(&current_exe).unwrap();

        // Write to install location
        // Tulis ke lokasi instalasi
        std::fs::write(&install_path, &binary_content).unwrap();

        // Add to user PATH
        // Tambahkan ke PATH pengguna
        let ps_command = format!(
            r#"[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";{}", "User")"#,
            install_dir.display()
        );
        let _ = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(&ps_command)
            .output();

        // Execute the installed binary with the same arguments
        // Jalankan binary yang diinstal dengan argumen yang sama
        let args: Vec<String> = std::env::args().collect();
        let mut cmd = std::process::Command::new(&install_path);
        cmd.args(&args[1..]);
        let _ = cmd.spawn().unwrap();

        std::process::exit(0);
    }

    util::setup_logging();

    let cli = Cli::parse();
    let _monitor = cli.monitor;
    let _verbose = cli.verbose;
    
    let cmd = match cli.command {
        Some(c) => c,
        None => {
            Cli::parse_and_run()?
        }
    };

    match cmd {
        octaskly::cmd::Command::Dispatcher {
            bind,
            port,
            workdir,
            max_workers: _,
            task_timeout: _,
            p2p_enabled: _,
            discovery_port: _,
            ui: _ui,
        } => {
            if _monitor {
                info!("[DISPATCHER] Monitor mode enabled");
            }
            run_dispatcher(&bind, port, workdir).await?;
        }
        octaskly::cmd::Command::Worker {
            name,
            dispatcher: _,
            dispatcher_port: _,
            max_jobs,
            cpu_cores: _,
            memory_mb: _,
            gpu: _,
            allow_shell,
        } => {
            if _monitor {
                info!("[WORKER] Monitor mode enabled");
            }
            run_worker(&name, allow_shell, max_jobs).await?;
        }
        _ => {
            eprintln!("Usage: octaskly <dispatcher | worker | d | w>");
            std::process::exit(1);
        }
    }

    Ok(())
}


async fn run_dispatcher(bind: &str, port: u16, workdir: PathBuf) -> Result<()> {
    // Initialize dispatcher with state management
    // Inisialisasi dispatcher dengan manajemen status
    info!("[DISPATCHER] Starting Octaskly Dispatcher on {}:{}", bind, port);

    let dispatcher_state = Arc::new(DispatcherState::new("dispatcher".to_string(), port));
    let scheduler = Arc::new(Scheduler::new());
    let active_tasks: Arc<RwLock<std::collections::HashMap<String, String>>> = 
        Arc::new(RwLock::new(std::collections::HashMap::new()));

    // Initialize P2P peer discovery and task distribution
    // Inisialisasi penemuan peer P2P dan distribusi task
    let p2p_network = match start_p2p_discovery(
        format!("dispatcher-{}", port),
        "Octaskly-Dispatcher".to_string(),
        5555,
    ).await {
        Ok(network) => {
            info!("[P2P] P2P network initialized successfully");
            Some(network)
        }
        Err(e) => {
            warn!("[P2P] P2P initialization warning: {}", e);
            None
        }
    };

    // Initialize P2P task distributor for resource sharing
    // Inisialisasi distributor task P2P untuk berbagi resource
    // Default: 4 CPU cores, 8GB RAM, no GPU, 4 task slots
    // Default: 4 core CPU, 8GB RAM, tidak ada GPU, 4 slot task
    let p2p_distributor = Arc::new(octaskly::P2PDistributor::new(
        format!("dispatcher-{}", port),
        4,           // CPU cores (default estimate)
        8192,        // RAM in MB (8GB default)
        false,       // GPU available
        4,           // Task slots
    ));

    // Create work directory if not exists
    // Buat direktori kerja jika belum ada
    util::ensure_dir(&workdir).await?;

    // Create network listener on specified address and port
    // Buat listener jaringan pada alamat dan port yang ditentukan
    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).await?;
    info!("[DISPATCHER] Listening on {}", addr);

    let listener = Arc::new(listener);

    info!("[DISPATCHER] Ready. Waiting for worker connections...");

    // Spawn task to handle incoming connections from workers
    // Jalankan task untuk menangani koneksi masuk dari worker
    let listener_clone = listener.clone();
    let scheduler_clone = scheduler.clone();
    let dispatcher_state_clone = dispatcher_state.clone();
    let active_tasks_clone = active_tasks.clone();
    
    tokio::spawn(async move {
        loop {
            match listener_clone.accept().await {
                Ok((stream, peer_addr)) => {
                    debug!("[DISPATCHER] Accepted connection from {}", peer_addr);
                    
                    let scheduler = scheduler_clone.clone();
                    let dispatcher_state = dispatcher_state_clone.clone();
                    let active_tasks = active_tasks_clone.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = 
                            octaskly::transport::Transport::handle_connection(
                                stream,
                                move |msg| {
                                    let scheduler = scheduler.clone();
                                    let dispatcher_state = dispatcher_state.clone();
                                    let active_tasks = active_tasks.clone();
                                    
                                    Box::pin(async move {
                                        handle_dispatcher_message(
                                            msg,
                                            &scheduler,
                                            &dispatcher_state,
                                            &active_tasks,
                                        )
                                        .await
                                    })
                                }
                            ).await 
                        {
                            error!("Connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    });

    // Scheduler loop - assign tasks to idle workers
    // Loop penjadwal - tugaskan tugas ke worker yang menganggur
    let scheduler_clone = scheduler.clone();
    let active_tasks_clone = active_tasks.clone();
    
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(500));
        
        loop {
            interval.tick().await;
            
            if let Some((task, mut worker)) = scheduler_clone.schedule_next_task().await {
                debug!("[SCHEDULER] Assigning task {} to worker {}", task.id, worker.id);
                
                // Mark task as assigned
                active_tasks_clone.write().await.insert(task.id.clone(), worker.id.clone());
                
                // Update worker current jobs
                worker.current_jobs += 1;
                scheduler_clone.update_worker(&worker.id, worker.clone()).await;
                
                // Try to send task to worker
                let worker_addr = format!("{}:{}", worker.address, worker.port);
                if let Ok(socket_addr) = worker_addr.parse::<SocketAddr>() {
                    let message = Message::AssignTask(task.clone());
                    if let Err(e) = octaskly::transport::Transport::new().send_message(socket_addr, &message).await {
                        warn!("Failed to send task to worker {}: {}", worker.id, e);
                        // Requeue task
                        scheduler_clone.enqueue(task).await;
                    }
                }
            }
        }
    });

    // Heartbeat cleanup loop - remove offline workers
    // Loop pembersihan detak jantung - hapus worker yang offline
    let scheduler_clone = scheduler.clone();
    
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        
        loop {
            interval.tick().await;
            scheduler_clone.cleanup_offline_workers(30).await;
        }
    });

    // P2P task distribution management loop
    // Loop manajemen distribusi task P2P
    if let Some(_p2p_net) = p2p_network.clone() {
        let p2p_distributor_clone = p2p_distributor.clone();
        let p2p_network_clone = p2p_network.clone();
        
        tokio::spawn(async move {
            if let Some(p2p_net) = p2p_network_clone {
                if let Err(e) = manage_p2p_distribution(
                    p2p_distributor_clone,
                    p2p_net,
                    1,
                ).await {
                    error!("[P2P] Task distribution error: {}", e);
                }
            }
        });
    }

    // P2P peer discovery and resource updates
    // Penemuan peer P2P dan pembaruan resource
    let p2p_distributor_clone = p2p_distributor.clone();
    
    tokio::spawn(async move {
        if let Err(e) = handle_p2p_peer_updates(
            p2p_distributor_clone,
            5,
        ).await {
            warn!("[P2P] Peer update error: {}", e);
        }
    });

    // Handle graceful shutdown
    // Tangani penutupan yang elegan
    tokio::signal::ctrl_c().await?;
    info!("[DISPATCHER] Shutting down gracefully...");

    Ok(())
}

// Handle incoming messages from workers at dispatcher
// Tangani pesan masuk dari worker di dispatcher
async fn handle_dispatcher_message(
    msg: Message,
    scheduler: &Scheduler,
    dispatcher_state: &DispatcherState,
    _active_tasks: &Arc<RwLock<std::collections::HashMap<String, String>>>,
) -> Result<()> {
    match msg {
        // Register worker when it announces itself
        // Daftarkan worker ketika mengumumkan dirinya
        Message::WorkerAnnounce(worker_info) => {
            info!("[DISPATCHER] Worker registered: {} ({}:{})", worker_info.name, worker_info.address, worker_info.port);
            scheduler.register_worker(worker_info).await;
        }
        
        // Task completion notification from worker
        // Notifikasi penyelesaian tugas dari worker
        Message::TaskCompleted(result) => {
            info!("[DISPATCHER] Task {} completed - status: {:?}", result.task_id, result.status);
            dispatcher_state.store_result(result.clone()).await;
            scheduler.worker_job_completed(&result.worker_id).await;
        }
        
        Message::TaskProgress { task_id, progress } => {
            debug!("[DISPATCHER] Task {} progress: {:.1}%", task_id, progress * 100.0);
        }
        
        // P2P: Resource availability announcement
        // P2P: Pengumuman ketersediaan resource
        Message::ResourceAnnounce(resources) => {
            info!(
                "[P2P] Resource announced from {}: CPU={}, RAM={}MB, GPU={}, Slots={}",
                resources.peer_id, resources.cpu_cores, resources.ram_mb, resources.gpu_available, resources.available_slots
            );
        }
        
        // P2P: Peer discovery request
        // P2P: Permintaan penemuan peer
        Message::PeerDiscoveryRequest { requester_id, timestamp } => {
            info!(
                "[P2P] Peer discovery request from {} at {}",
                requester_id, timestamp
            );
        }
        
        // P2P: Peer discovery response
        // P2P: Respons penemuan peer
        Message::PeerDiscoveryResponse { responder_id, resources } => {
            info!(
                "[P2P] Peer discovery response from {}: {}",
                responder_id, resources.peer_id
            );
        }
        
        // Worker heartbeat for health monitoring
        // Detak jantung worker untuk pemantauan kesehatan
        Message::Heartbeat { worker_id, timestamp: _ } => {
            debug!("[DISPATCHER] Heartbeat received from {}", worker_id);
            // Update worker last_heartbeat in scheduler
            // Perbarui last_heartbeat worker di penjadwal
            let workers = scheduler.get_workers().await;
            if let Some(mut worker) = workers.iter().find(|w| w.id == worker_id).cloned() {
                worker.last_heartbeat = chrono::Local::now().timestamp();
                scheduler.update_worker(&worker_id, worker).await;
            }
        }
        
        _ => {
            warn!("Unexpected message type: {:?}", msg);
        }
    }
    
    Ok(())
}

// Worker process initialization and main loop
// Inisialisasi proses worker dan loop utama
async fn run_worker(name: &str, allow_shell: bool, max_jobs: usize) -> Result<()> {
    info!("[WORKER] Starting Worker '{}' with max_jobs={}", name, max_jobs);

    let local_ip = util::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    let port = find_available_port(7879).await?;

    let worker_state = Arc::new(WorkerState::new(name.to_string(), port));
    let executor = Arc::new(Executor::new(PathBuf::from("./work"), allow_shell));

    let worker_info = WorkerInfo::new(
        name.to_string(),
        local_ip.clone(),
        port,
        max_jobs,
    );

    info!(
        "[WORKER] Registered at {}:{}",
        local_ip, port
    );
    info!("[WORKER] Waiting for dispatcher assignment...");

    // Start listening for incoming connections from dispatcher
    // Mulai mendengarkan koneksi masuk dari dispatcher
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("[WORKER] Listening on {}", addr);

    let listener = Arc::new(listener);
    let worker_info_announced = Arc::new(RwLock::new(false));
    let worker_info_to_announce = worker_info.clone();

    // Spawn connection handler task
    // Jalankan task penanganan koneksi
    let listener_clone = listener.clone();
    let worker_state_clone = worker_state.clone();
    let executor_clone = executor.clone();
    let worker_info_announced_clone = worker_info_announced.clone();
    let worker_info_for_handler = worker_info_to_announce.clone();
    
    tokio::spawn(async move {
        loop {
            match listener_clone.accept().await {
                Ok((stream, peer_addr)) => {
                    debug!("[WORKER] Connection established with dispatcher at {}", peer_addr);
                    
                    // Announce worker to dispatcher if not already done
                    // Umumkan worker ke dispatcher jika belum dilakukan
                    if !*worker_info_announced_clone.read().await {
                        *worker_info_announced_clone.write().await = true;
                        let announce_msg = Message::WorkerAnnounce(worker_info_for_handler.clone());
                        let announce_addr = peer_addr;
                        
                        if let Err(e) = octaskly::transport::Transport::new().send_message(announce_addr, &announce_msg).await {
                            warn!("Failed to announce worker: {}", e);
                        }
                    }
                    
                    let worker_state = worker_state_clone.clone();
                    let executor = executor_clone.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = 
                            octaskly::transport::Transport::handle_connection(
                                stream,
                                move |msg| {
                                    let worker_state = worker_state.clone();
                                    let executor = executor.clone();
                                    
                                    Box::pin(async move {
                                        handle_worker_message(msg, &worker_state, &executor, peer_addr).await
                                    })
                                }
                            ).await 
                        {
                            error!("Worker connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    });

    // Heartbeat loop - send periodic heartbeats to dispatcher
    // Loop detak jantung - kirim detak jantung berkala ke dispatcher
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            debug!("[WORKER] Heartbeat check");
            // Note: In production, we'd track dispatcher address and send heartbeat
            // Catatan: Dalam produksi, kami akan melacak alamat dispatcher dan mengirim heartbeat
        }
    });

    // Keep running
    // Tetap berjalan
    tokio::signal::ctrl_c().await?;
    info!("[WORKER] Shutting down gracefully...");
    
    Ok(())
}

// Handle task execution messages on worker
// Tangani pesan eksekusi tugas di worker
async fn handle_worker_message(
    msg: Message,
    worker_state: &WorkerState,
    executor: &Executor,
    dispatcher_addr: SocketAddr,
) -> Result<()> {
    match msg {
        // Execute assigned task from dispatcher
        // Jalankan tugas yang ditugaskan dari dispatcher
        Message::AssignTask(task) => {
            info!("[WORKER] Task received for execution: {}", task.id);
            
            let task_id = task.id.clone();
            worker_state.set_current_task(Some(task.clone())).await;
            
            // Execute task with timeout protection
            // Jalankan tugas dengan perlindungan timeout
            match executor.execute_with_timeout(&task).await {
                Ok(result) => {
                    info!("[WORKER] Task {} execution completed successfully", task_id);
                    
                    let task_result = octaskly::protocol::TaskResult {
                        task_id: task_id.clone(),
                        worker_id: "unknown".to_string(),
                        status: result.status,
                        stdout: result.stdout,
                        stderr: result.stderr,
                        exit_code: result.exit_code,
                        duration_ms: result.duration_ms,
                        completed_at: chrono::Local::now().timestamp(),
                    };
                    
                    // Send result back to dispatcher
                    // Kirim hasil kembali ke dispatcher
                    let result_msg = Message::TaskCompleted(task_result);
                    if let Err(e) = octaskly::transport::Transport::new().send_message(dispatcher_addr, &result_msg).await {
                        error!("[WORKER] Failed to send task result: {}", e);
                    }
                    
                    worker_state.set_current_task(None).await;
                }
                Err(e) => {
                    error!("Task execution failed: {}", e);
                    worker_state.set_current_task(None).await;
                }
            }
        }
        
        // Task cancellation request
        // Permintaan pembatalan tugas
        Message::CancelTask { task_id } => {
            info!("[WORKER] Cancel request received for task: {}", task_id);
            worker_state.set_current_task(None).await;
        }
        
        // P2P: Shared task from peer
        // P2P: Task bersama dari peer
        Message::P2PShareTask { task, requester_id } => {
            info!("[P2P] Shared task received from {}: {}", requester_id, task.id);
            
            // For now, execute like normal task
            let task_id = task.id.clone();
            worker_state.set_current_task(Some(task.clone())).await;
            
            match executor.execute_with_timeout(&task).await {
                Ok(result) => {
                    info!("[P2P] Shared task {} completed", task_id);
                    let task_result = octaskly::protocol::TaskResult {
                        task_id: task_id.clone(),
                        worker_id: "unknown".to_string(),
                        status: result.status,
                        stdout: result.stdout,
                        stderr: result.stderr,
                        exit_code: result.exit_code,
                        duration_ms: result.duration_ms,
                        completed_at: chrono::Local::now().timestamp(),
                    };
                    
                    // Send result back to requester
                    let result_msg = Message::TaskCompleted(task_result);
                    let _ = octaskly::transport::Transport::new().send_message(dispatcher_addr, &result_msg).await;
                }
                Err(e) => {
                    error!("[P2P] Shared task execution failed: {}", e);
                }
            }
            
            worker_state.set_current_task(None).await;
        }
        
        // P2P: Peer discovery request
        // P2P: Permintaan penemuan peer
        Message::PeerDiscoveryRequest { requester_id, timestamp } => {
            info!(
                "[P2P] Discovery request from {} at {}",
                requester_id, timestamp
            );
        }
        
        _ => {
            warn!("Unexpected message type for worker: {:?}", msg);
        }
    }
    
    Ok(())
}

/// Find an available port starting from the given port
/// Cari port yang tersedia dimulai dari port yang diberikan
async fn find_available_port(start_port: u16) -> Result<u16> {
    for port in start_port..(start_port + 100) {
        let addr = format!("0.0.0.0:{}", port);
        match TcpListener::bind(&addr).await {
            Ok(_) => return Ok(port),
            Err(_) => continue,
        }
    }
    Err(anyhow::anyhow!("No available port found"))
}

/// Start P2P peer discovery and management
/// Mulai penemuan peer P2P dan manajemen
async fn start_p2p_discovery(
    peer_id: String,
    peer_name: String,
    broadcast_port: u16,
) -> Result<Arc<octaskly::P2PNetwork>> {
    let p2p_network = Arc::new(octaskly::P2PNetwork::new(
        peer_id.clone(),
        peer_name.clone(),
        broadcast_port,
    )?);

    // Start mDNS service discovery
    p2p_network.start_mdns_discovery().await?;

    // Start discovery listener on broadcast port
    p2p_network.start_discovery_listener(5555).await?;

    // Start periodic peer discovery announcements
    p2p_network.start_periodic_discovery(10).await?;

    // Announce this peer to the network
    p2p_network.announce_peer().await?;

    info!(
        "[P2P] Peer discovery started for: {} ({})",
        peer_name, peer_id
    );

    Ok(p2p_network)
}

/// Manage P2P task distribution
/// Kelola distribusi task P2P
async fn manage_p2p_distribution(
    distributor: Arc<octaskly::P2PDistributor>,
    _p2p_network: Arc<octaskly::P2PNetwork>,
    dispatcher_interval: u64,
) -> Result<()> {
    // Periodic task distribution loop
    let mut ticker = interval(Duration::from_secs(dispatcher_interval));

    loop {
        ticker.tick().await;

        // Cleanup stale peer information
        if let Err(e) = distributor.cleanup_stale_peers(30).await {
            warn!("[P2P] Peer cleanup error: {}", e);
        }

        // Process pending tasks and distribute to available peers
        while let Some(task) = distributor.get_next_task().await {
            // Find best peer for this task
            let best_peer = distributor.find_best_peer(&task, 1, 256, false).await;

            match best_peer {
                Some(peer_id) => {
                    if let Err(e) = distributor.assign_to_peer(task.id.clone(), peer_id.clone()).await {
                        error!("[P2P] Failed to assign task {} to peer {}: {}", task.id, peer_id, e);
                        distributor.enqueue_p2p_task(task).await?;
                    } else {
                        info!("[P2P] Task {} assigned to peer {}", task.id, peer_id);
                    }
                }
                None => {
                    warn!("[P2P] No suitable peer found for task {}, re-queueing", task.id);
                    distributor.enqueue_p2p_task(task).await?;
                }
            }
        }

        // Log distribution stats periodically
        let pending = distributor.pending_count().await;
        let active = distributor.assignment_count().await;
        let peers = distributor.get_all_peers().await;

        debug!(
            "[P2P] Distribution stats - Pending: {}, Active: {}, Peers: {}",
            pending, active, peers.len()
        );
    }
}

/// Handle P2P peer discovery and resource updates
/// Tangani penemuan peer P2P dan pembaruan resource
async fn handle_p2p_peer_updates(
    distributor: Arc<octaskly::P2PDistributor>,
    discovery_interval: u64,
) -> Result<()> {
    let mut ticker = interval(Duration::from_secs(discovery_interval));

    loop {
        ticker.tick().await;

        // Log current peer status
        let peers = distributor.get_all_peers().await;
        for peer in peers {
            debug!(
                "[P2P] Peer: {} - CPU: {}, RAM: {}MB, GPU: {}, Slots: {}",
                peer.peer_id,
                peer.cpu_cores,
                peer.ram_mb,
                peer.gpu_available,
                peer.available_slots
            );
        }
    }
}

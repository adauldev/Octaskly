use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Validates input arguments for commands
/// Memvalidasi argumen input untuk perintah
pub struct Validator;

impl Validator {
    /// Validate dispatcher arguments
    pub fn dispatcher(bind: &str, port: u16, max_workers: usize) -> Result<(), String> {
        // Validate bind address
        if bind.is_empty() {
            return Err("Bind address cannot be empty".to_string());
        }

        // Validate port
        if port == 0 {
            return Err("Port must be greater than 0".to_string());
        }
        if port < 1024 {
            return Err("Port must be 1024 or higher (non-privileged)".to_string());
        }

        // Validate max workers
        if max_workers == 0 {
            return Err("Max workers must be at least 1".to_string());
        }
        if max_workers > 1000 {
            return Err("Max workers exceeds reasonable limit (1000)".to_string());
        }

        Ok(())
    }

    /// Validate worker arguments
    pub fn worker(name: &str, dispatcher: &str, dispatcher_port: u16, max_jobs: usize) -> Result<(), String> {
        // Validate name
        if name.is_empty() {
            return Err("Worker name cannot be empty".to_string());
        }
        if name.len() > 256 {
            return Err("Worker name exceeds max length (256 chars)".to_string());
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("Worker name must contain only alphanumeric characters, hyphens, or underscores".to_string());
        }

        // Validate dispatcher address
        if dispatcher.is_empty() {
            return Err("Dispatcher address cannot be empty".to_string());
        }

        // Validate dispatcher port
        if dispatcher_port == 0 {
            return Err("Dispatcher port must be greater than 0".to_string());
        }

        // Validate max jobs
        if max_jobs == 0 {
            return Err("Max jobs must be at least 1".to_string());
        }
        if max_jobs > 1000 {
            return Err("Max jobs exceeds reasonable limit (1000)".to_string());
        }

        Ok(())
    }
}

/// Distributed task orchestration with P2P resource sharing
#[derive(Parser, Debug)]
#[command(name = "octaskly")]
#[command(about = "Distributed Task Orchestration with P2P Resource Sharing")]
#[command(long_about = "Octaskly - Distributed task scheduler for local networks\n\nTwo modes available:\n  dispatcher  - Central task coordinator\n  worker      - Task execution node")]
#[command(version = "1.0.0")]
#[command(author = "Octaskly Contributors")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Enable real-time monitoring dashboard (TUI)
    #[arg(global = true, long)]
    pub monitor: bool,

    /// Show verbose debug output
    #[arg(global = true, short = 'v', long)]
    pub verbose: bool,
}

/// Available application commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start as dispatcher (central task coordinator)
    #[command(about = "Central task coordinator - manages workers and distributes tasks")]
    Dispatcher {
        /// Bind address for worker connections [default: 0.0.0.0]
        #[arg(short = 'b', long, default_value = "0.0.0.0")]
        bind: String,

        /// TCP port for dispatcher to listen on [default: 7878]
        #[arg(short = 'p', long, default_value = "7878")]
        port: u16,

        /// Directory for storing task data and results [default: ./tasks]
        #[arg(long, default_value = "./tasks")]
        workdir: PathBuf,

        /// Maximum number of worker connections [default: 10]
        #[arg(long, default_value = "10")]
        max_workers: usize,

        /// Task execution timeout in seconds [default: 300]
        #[arg(short = 't', long, default_value = "300")]
        task_timeout: u64,

        /// Enable P2P peer-to-peer task distribution [default: true]
        #[arg(long, default_value = "true")]
        p2p_enabled: bool,

        /// UDP port for P2P peer discovery and announcements [default: 5555]
        #[arg(long, default_value = "5555")]
        discovery_port: u16,

        /// Enable interactive terminal UI dashboard
        #[arg(long)]
        ui: bool,
    },

    /// Start as worker (task execution node)
    #[command(about = "Task execution node - receives and executes tasks")]
    Worker {
        /// Unique worker identifier within cluster (required)
        #[arg(short = 'n', long)]
        name: String,

        /// Address of dispatcher to connect to [default: localhost]
        #[arg(short = 'd', long, default_value = "localhost")]
        dispatcher: String,

        /// Port of dispatcher to connect to [default: 7878]
        #[arg(short = 'p', long, default_value = "7878")]
        dispatcher_port: u16,

        /// Maximum number of concurrent tasks this worker can execute [default: 4]
        #[arg(short = 'j', long, default_value = "4")]
        max_jobs: usize,

        /// Number of CPU cores available on this worker (auto-detect if not set)
        #[arg(short = 'c', long)]
        cpu_cores: Option<usize>,

        /// Amount of RAM available on this worker in MB (auto-detect if not set)
        #[arg(short = 'm', long)]
        memory_mb: Option<u64>,

        /// GPU hardware availability
        #[arg(long, default_value = "false")]
        gpu: bool,

        /// Allow execution of shell commands on this worker
        #[arg(long, default_value = "true")]
        allow_shell: bool,
    },

    /// Quick dispatcher launch
    #[command(about = "Quick dispatcher (same as: dispatcher -b 0.0.0.0 -p 7878 --ui)")]
    D {
        #[arg(long, default_value = "0.0.0.0")]
        bind: String,
        #[arg(long, default_value = "7878")]
        port: u16,
        #[arg(long)]
        ui: bool,
    },

    /// Quick worker launch
    #[command(about = "Quick worker (same as: worker -n NAME -j 4)")]
    W {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "4")]
        max_jobs: usize,
    },
}

impl Cli {
    /// Parse CLI arguments and normalize command shortcuts with validation
    pub fn parse_and_run() -> Result<Command, anyhow::Error> {
        let cli = Self::parse();
        
        let cmd = match cli.command {
            Some(Command::Dispatcher { 
                bind, 
                port, 
                workdir, 
                max_workers, 
                task_timeout, 
                p2p_enabled, 
                discovery_port, 
                ui 
            }) => {
                // Validate dispatcher arguments
                if let Err(e) = Validator::dispatcher(&bind, port, max_workers) {
                    eprintln!("❌ Dispatcher validation failed: {}", e);
                    std::process::exit(1);
                }
                Command::Dispatcher {
                    bind,
                    port,
                    workdir,
                    max_workers,
                    task_timeout,
                    p2p_enabled,
                    discovery_port,
                    ui,
                }
            }
            Some(Command::Worker { 
                name, 
                dispatcher, 
                dispatcher_port, 
                max_jobs, 
                cpu_cores, 
                memory_mb, 
                gpu, 
                allow_shell 
            }) => {
                // Validate worker arguments
                if let Err(e) = Validator::worker(&name, &dispatcher, dispatcher_port, max_jobs) {
                    eprintln!("❌ Worker validation failed: {}", e);
                    std::process::exit(1);
                }
                Command::Worker {
                    name,
                    dispatcher,
                    dispatcher_port,
                    max_jobs,
                    cpu_cores,
                    memory_mb,
                    gpu,
                    allow_shell,
                }
            }
            Some(Command::D { bind, port, ui }) => {
                // Quick dispatcher - validate
                if let Err(e) = Validator::dispatcher(&bind, port, 10) {
                    eprintln!("❌ Dispatcher validation failed: {}", e);
                    std::process::exit(1);
                }
                Command::Dispatcher {
                    bind,
                    port,
                    workdir: PathBuf::from("./tasks"),
                    max_workers: 10,
                    task_timeout: 300,
                    p2p_enabled: true,
                    discovery_port: 5555,
                    ui,
                }
            }
            Some(Command::W { name, max_jobs }) => {
                // Quick worker - validate
                if let Err(e) = Validator::worker(&name, "localhost", 7878, max_jobs) {
                    eprintln!("❌ Worker validation failed: {}", e);
                    std::process::exit(1);
                }
                Command::Worker {
                    name,
                    dispatcher: "localhost".to_string(),
                    dispatcher_port: 7878,
                    max_jobs,
                    cpu_cores: None,
                    memory_mb: None,
                    gpu: false,
                    allow_shell: true,
                }
            }
            None => {
                Self::show_default_help();
                std::process::exit(1);
            }
        };

        Ok(cmd)
    }

    /// Display minimalist default help message
    fn show_default_help() {
        println!();
        println!("octaskly v1.0.0 | Distributed Task Orchestration with P2P Sharing");
        println!();
        println!("USAGE:");
        println!("  octaskly dispatcher [OPTIONS]   Central task coordinator");
        println!("  octaskly worker [OPTIONS]       Task execution node");
        println!("  octaskly d [OPTIONS]            Quick dispatcher");
        println!("  octaskly w [OPTIONS]            Quick worker");
        println!();
        println!("OPTIONS (Global):");
        println!("  --monitor                       Enable real-time TUI dashboard");
        println!("  -v, --verbose                   Show debug output");
        println!("  -h, --help                      Show detailed help");
        println!("  -V, --version                   Show version");
        println!();
        println!("DISPATCHER OPTIONS:");
        println!("  -b, --bind ADDR                 Bind address [default: 0.0.0.0]");
        println!("  -p, --port NUM                  Port number [default: 7878]");
        println!("  -t, --task-timeout SECS         Timeout in seconds [default: 300]");
        println!("  --max-workers NUM               Maximum workers [default: 10]");
        println!("  --workdir PATH                  Task directory [default: ./tasks]");
        println!("  --p2p-enabled BOOL              Enable P2P [default: true]");
        println!("  --discovery-port NUM            P2P port [default: 5555]");
        println!("  --ui                            Enable terminal UI dashboard");
        println!();
        println!("WORKER OPTIONS:");
        println!("  -n, --name NAME                 Unique worker name (required)");
        println!("  -d, --dispatcher ADDR           Dispatcher address [default: localhost]");
        println!("  -p, --dispatcher-port NUM       Dispatcher port [default: 7878]");
        println!("  -j, --max-jobs NUM              Max concurrent jobs [default: 4]");
        println!("  -c, --cpu-cores NUM             CPU cores (auto-detect if empty)");
        println!("  -m, --memory-mb NUM             RAM in MB (auto-detect if empty)");
        println!("  --gpu BOOL                      GPU available [default: false]");
        println!("  --allow-shell BOOL              Allow shell exec [default: true]");
        println!();
        println!("EXAMPLES:");
        println!("  octaskly dispatcher --port 7878 --ui");
        println!("  octaskly worker -n worker-01");
        println!("  octaskly d --monitor");
        println!("  octaskly w -n prod-worker -d 192.168.1.10");
        println!();
        println!("Use 'octaskly --help' for full documentation");
        println!();
    }
}

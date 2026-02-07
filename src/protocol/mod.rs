use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a compute task to be executed
/// Merepresentasikan tugas komputasi yang akan dieksekusi
///
/// A task encapsulates a command, its parameters, input/output files,
/// and execution constraints. Tasks are the fundamental unit of work
/// distributed by the dispatcher to workers.
/// 
/// Tugas merangkum perintah, parameternya, file input/output, dan batasan
/// eksekusi. Tugas adalah unit kerja fundamental yang didistribusikan oleh
/// dispatcher ke worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier (UUID)
    /// Pengenal unik tugas (UUID)
    pub id: String,
    
    /// Command string to execute (shell or executable)
    /// String perintah untuk dieksekusi (shell atau executable)
    pub command: String,
    
    /// Input files required for task execution
    /// File input yang diperlukan untuk eksekusi tugas
    pub inputs: Vec<String>,
    
    /// Output files produced by task execution
    /// File output yang dihasilkan oleh eksekusi tugas
    pub outputs: Vec<String>,
    
    /// Maximum execution time in seconds
    /// Waktu eksekusi maksimum dalam detik
    pub timeout: u64,
    
    /// Environment variables for task execution
    /// Variabel lingkungan untuk eksekusi tugas
    pub env: HashMap<String, String>,
    
    /// Task creation timestamp (Unix epoch)
    /// Stempel waktu pembuatan tugas (Unix epoch)
    pub created_at: i64,
}

impl Task {
    /// Create a new task with sensible defaults
    /// Buat tugas baru dengan nilai default yang masuk akal
    ///
    /// Sets default timeout to 10 minutes and generates UUID
    /// Menetapkan timeout default ke 10 menit dan membuat UUID
    pub fn new(command: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            command,
            inputs: Vec::new(),
            outputs: Vec::new(),
            timeout: 600, // 10 minutes default
            env: HashMap::new(),
            created_at: chrono::Local::now().timestamp(),
        }
    }
}

/// Represents the result of task execution
/// Merepresentasikan hasil eksekusi tugas
///
/// Contains execution output, status, timing information, and exit code.
/// Results are collected by workers and reported back to dispatcher for
/// aggregation and storage.
/// 
/// Berisi output eksekusi, status, informasi waktu, dan kode keluar.
/// Hasil dikumpulkan oleh worker dan dilaporkan kembali ke dispatcher
/// untuk agregasi dan penyimpanan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Reference to source task
    /// Referensi ke tugas sumber
    pub task_id: String,
    
    /// Worker that executed the task
    /// Worker yang menjalankan tugas
    pub worker_id: String,
    
    /// Final task status (completed, failed, etc)
    /// Status tugas akhir (selesai, gagal, dll)
    pub status: TaskStatus,
    
    /// Standard output from task execution
    /// Output standar dari eksekusi tugas
    pub stdout: String,
    
    /// Standard error output from task execution
    /// Output kesalahan standar dari eksekusi tugas
    pub stderr: String,
    
    /// Process exit code (0 = success, non-zero = error)
    /// Kode keluar proses (0 = sukses, non-nol = kesalahan)
    pub exit_code: Option<i32>,
    
    /// Total execution time in milliseconds
    /// Total waktu eksekusi dalam milidetik
    pub duration_ms: u64,
    
    /// Completion timestamp (Unix epoch)
    /// Stempel waktu penyelesaian (Unix epoch)
    pub completed_at: i64,
}

/// Task execution status enumeration
/// Enumerasi status eksekusi tugas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Waiting for worker assignment
    /// Menunggu penugasan worker
    Pending,
    
    /// Currently executing on worker
    /// Sedang dieksekusi pada worker
    Running,
    
    /// Completed successfully
    /// Selesai dengan sukses
    Completed,
    
    /// Execution failed (non-zero exit)
    /// Eksekusi gagal (keluar non-nol)
    Failed,
    
    /// Explicitly cancelled by user
    /// Dibatalkan secara eksplisit oleh pengguna
    Cancelled,
    
    /// Exceeded timeout threshold
    /// Melampaui ambang timeout
    TimedOut,
}

/// Represents a worker node in the cluster
/// Merepresentasikan node worker dalam cluster
///
/// Workers register with dispatcher and periodically send heartbeats.
/// Dispatcher tracks worker availability and task capacity to make
/// intelligent scheduling decisions.
/// 
/// Worker mendaftar dengan dispatcher dan secara berkala mengirim
/// heartbeat. Dispatcher melacak ketersediaan worker dan kapasitas
/// tugas untuk membuat keputusan penjadwalan yang cerdas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    /// Unique worker identifier (UUID)
    /// Pengenal unik worker (UUID)
    pub id: String,
    
    /// Human-readable worker name (e.g., "worker-prod-01")
    /// Nama worker yang dapat dibaca manusia (mis., "worker-prod-01")
    pub name: String,
    
    /// Network address where worker is reachable
    /// Alamat jaringan tempat worker dapat dijangkau
    pub address: String,
    
    /// Network port used by worker
    /// Port jaringan yang digunakan oleh worker
    pub port: u16,
    
    /// Maximum concurrent jobs this worker can execute
    /// Jumlah maksimum pekerjaan bersamaan yang dapat dijalankan worker ini
    pub max_jobs: usize,
    
    /// Current number of jobs being executed
    /// Jumlah saat ini pekerjaan yang sedang dijalankan
    pub current_jobs: usize,
    
    /// Whether this worker accepts shell commands
    /// Apakah worker ini menerima perintah shell
    pub allow_shell: bool,
    
    /// Timestamp of last heartbeat from worker (Unix epoch)
    /// Stempel waktu heartbeat terakhir dari worker (Unix epoch)
    pub last_heartbeat: i64,
    
    /// Operating system platform (linux, windows, macos)
    /// Platform sistem operasi (linux, windows, macos)
    pub platform: String,
}

impl WorkerInfo {
    /// Create new worker information record
    /// Buat catatan informasi worker baru
    pub fn new(name: String, address: String, port: u16, max_jobs: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            address,
            port,
            max_jobs,
            current_jobs: 0,
            allow_shell: true,
            last_heartbeat: chrono::Local::now().timestamp(),
            platform: std::env::consts::OS.to_string(),
        }
    }

    /// Check if worker has available job slots
    /// Periksa apakah worker memiliki slot pekerjaan tersedia
    pub fn is_idle(&self) -> bool {
        self.current_jobs < self.max_jobs
    }
}

/// Resource availability for P2P task sharing
/// Ketersediaan resource untuk berbagi task P2P
///
/// Peers periodically broadcast their available resources (CPU, RAM, GPU)
/// on the local network. This information enables intelligent P2P task
/// distribution based on resource requirements.
/// 
/// Peer secara berkala menyiarkan resource yang tersedia (CPU, RAM, GPU)
/// di jaringan lokal. Informasi ini memungkinkan distribusi task P2P yang
/// cerdas berdasarkan kebutuhan resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAvailability {
    /// Identifier of peer advertising resources
    /// Pengenal peer yang mengiklankan resource
    pub peer_id: String,
    
    /// Number of available CPU cores
    /// Jumlah inti CPU yang tersedia
    pub cpu_cores: usize,
    
    /// Available RAM in megabytes
    /// RAM yang tersedia dalam megabyte
    pub ram_mb: u64,
    
    /// GPU device availability
    /// Ketersediaan perangkat GPU
    pub gpu_available: bool,
    
    /// Current system load percentage (0-100)
    /// Persentase beban sistem saat ini (0-100)
    pub current_load: f32,
    
    /// Number of task execution slots available
    /// Jumlah slot eksekusi tugas yang tersedia
    pub available_slots: usize,
    
    /// Timestamp when this announcement was made (Unix epoch)
    /// Stempel waktu ketika pengumuman ini dibuat (Unix epoch)
    pub timestamp: i64,
}

impl ResourceAvailability {
    pub fn new(
        peer_id: String,
        cpu_cores: usize,
        ram_mb: u64,
        gpu_available: bool,
        available_slots: usize,
    ) -> Self {
        Self {
            peer_id,
            cpu_cores,
            ram_mb,
            gpu_available,
            current_load: 0.0,
            available_slots,
            timestamp: chrono::Local::now().timestamp(),
        }
    }
}

/// Protocol messages for communication
/// Pesan protokol untuk komunikasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// Worker announces itself to dispatcher
    WorkerAnnounce(WorkerInfo),
    
    /// Dispatcher assigns a task to worker
    AssignTask(Task),
    
    /// Worker reports task progress
    TaskProgress {
        task_id: String,
        progress: f32,
    },
    
    /// Worker reports task completion
    TaskCompleted(TaskResult),
    
    /// Heartbeat message
    Heartbeat {
        worker_id: String,
        timestamp: i64,
    },
    
    /// Cancel a task
    CancelTask {
        task_id: String,
    },
    
    /// Acknowledge message
    Ack {
        message_id: String,
    },
    
    /// P2P: Announce resource availability to peers
    /// P2P: Umumkan ketersediaan resource ke peer
    ResourceAnnounce(ResourceAvailability),
    
    /// P2P: Request task to be executed by a peer
    /// P2P: Minta task dijalankan oleh peer
    P2PShareTask {
        task: Task,
        requester_id: String,
    },
    
    /// P2P: Accept or reject shared task
    /// P2P: Terima atau tolak task bersama
    P2PTaskResponse {
        task_id: String,
        accepted: bool,
        reason: Option<String>,
    },
    
    /// P2P: Discover peers on network
    /// P2P: Temukan peer di jaringan
    PeerDiscoveryRequest {
        requester_id: String,
        timestamp: i64,
    },
    
    /// P2P: Response with peer information
    /// P2P: Respons dengan informasi peer
    PeerDiscoveryResponse {
        responder_id: String,
        resources: ResourceAvailability,
    },
}

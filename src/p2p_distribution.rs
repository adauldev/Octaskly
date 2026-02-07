// P2P Task Distribution Module
// Modul Distribusi Task P2P
//
// Handles task distribution across P2P network and resource matching
// Menangani distribusi task di jaringan P2P dan pencocokan resource

use crate::protocol::{Message, ResourceAvailability, Task};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// P2P Task Distribution Manager
/// Manajer Distribusi Task P2P
pub struct P2PDistributor {
    /// Local peer ID
    local_id: String,

    /// Available resources on this node
    resources: Arc<RwLock<ResourceAvailability>>,

    /// Known peers and their resources
    peer_resources: Arc<RwLock<HashMap<String, ResourceAvailability>>>,

    /// Tasks waiting for peer execution
    pending_tasks: Arc<RwLock<Vec<Task>>>,

    /// Task to peer assignment mapping
    task_assignments: Arc<RwLock<HashMap<String, String>>>,
}

impl P2PDistributor {
    /// Create new P2P distributor
    pub fn new(
        local_id: String,
        cpu_cores: usize,
        ram_mb: u64,
        gpu_available: bool,
        max_slots: usize,
    ) -> Self {
        let resources = ResourceAvailability::new(
            local_id.clone(),
            cpu_cores,
            ram_mb,
            gpu_available,
            max_slots,
        );

        Self {
            local_id,
            resources: Arc::new(RwLock::new(resources)),
            peer_resources: Arc::new(RwLock::new(HashMap::new())),
            pending_tasks: Arc::new(RwLock::new(Vec::new())),
            task_assignments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Update local resource availability
    /// Perbarui ketersediaan resource lokal
    pub async fn update_local_resources(
        &self,
        current_load: f32,
        available_slots: usize,
    ) -> Result<()> {
        let mut resources = self.resources.write().await;
        resources.current_load = current_load;
        resources.available_slots = available_slots;
        resources.timestamp = chrono::Local::now().timestamp();

        debug!(
            "[P2P] Local resources updated: load={}, slots={}",
            current_load, available_slots
        );

        Ok(())
    }

    /// Register peer resources
    /// Daftarkan resource peer
    pub async fn register_peer_resources(&self, resources: ResourceAvailability) -> Result<()> {
        let peer_id = resources.peer_id.clone();
        let mut peer_resources = self.peer_resources.write().await;

        if let Some(_existing) = peer_resources.get(&peer_id) {
            debug!("[P2P] Updating peer resources: {}", peer_id);
        } else {
            info!("[P2P] New peer resources registered: {}", peer_id);
        }

        peer_resources.insert(peer_id, resources);
        Ok(())
    }

    /// Get best peer for task based on resource requirements
    /// Dapatkan peer terbaik untuk task berdasarkan kebutuhan resource
    pub async fn find_best_peer(
        &self,
        _task: &Task,
        min_cpu: usize,
        min_ram_mb: u64,
        require_gpu: bool,
    ) -> Option<String> {
        let peer_resources = self.peer_resources.read().await;

        // Find peers matching requirements
        let mut suitable_peers: Vec<_> = peer_resources
            .values()
            .filter(|r| {
                r.cpu_cores >= min_cpu
                    && r.ram_mb >= min_ram_mb
                    && (!require_gpu || r.gpu_available)
                    && r.available_slots > 0
            })
            .collect();

        // Sort by availability (prefer less loaded peers)
        suitable_peers.sort_by(|a, b| a.current_load.partial_cmp(&b.current_load).unwrap());

        suitable_peers.first().map(|r| r.peer_id.clone())
    }

    /// Enqueue task for P2P distribution
    /// Antrekan task untuk distribusi P2P
    pub async fn enqueue_p2p_task(&self, task: Task) -> Result<()> {
        let mut pending = self.pending_tasks.write().await;
        pending.push(task.clone());

        info!("[P2P] Task enqueued for distribution: {}", task.id);
        Ok(())
    }

    /// Get next task to distribute
    /// Dapatkan task berikutnya untuk didistribusikan
    pub async fn get_next_task(&self) -> Option<Task> {
        let mut pending = self.pending_tasks.write().await;
        if !pending.is_empty() {
            Some(pending.remove(0))
        } else {
            None
        }
    }

    /// Assign task to peer
    /// Tugaskan task ke peer
    pub async fn assign_to_peer(&self, task_id: String, peer_id: String) -> Result<()> {
        let mut assignments = self.task_assignments.write().await;
        assignments.insert(task_id.clone(), peer_id.clone());

        let mut peer_res = self.peer_resources.write().await;
        if let Some(res) = peer_res.get_mut(&peer_id) {
            if res.available_slots > 0 {
                res.available_slots -= 1;
            }
        }

        info!(
            "[P2P] Task {} assigned to peer {}",
            task_id, peer_id
        );
        Ok(())
    }

    /// Mark task as completed by peer
    /// Tandai task selesai oleh peer
    pub async fn mark_task_completed(&self, task_id: String, peer_id: String) -> Result<()> {
        let mut assignments = self.task_assignments.write().await;
        assignments.remove(&task_id);

        let mut peer_res = self.peer_resources.write().await;
        if let Some(res) = peer_res.get_mut(&peer_id) {
            res.available_slots += 1;
        }

        info!(
            "[P2P] Task {} completed by peer {}",
            task_id, peer_id
        );
        Ok(())
    }

    /// Get all pending tasks
    /// Dapatkan semua task yang tertunda
    pub async fn get_pending_tasks(&self) -> Vec<Task> {
        let pending = self.pending_tasks.read().await;
        pending.clone()
    }

    /// Get peer resources
    /// Dapatkan resource peer
    pub async fn get_peer_resources(&self, peer_id: &str) -> Option<ResourceAvailability> {
        let peer_resources = self.peer_resources.read().await;
        peer_resources.get(peer_id).cloned()
    }

    /// Get all known peers with resources
    /// Dapatkan semua peer yang dikenal dengan resource
    pub async fn get_all_peers(&self) -> Vec<ResourceAvailability> {
        let peer_resources = self.peer_resources.read().await;
        peer_resources.values().cloned().collect()
    }

    /// Get pending queue size
    /// Dapatkan ukuran antrian yang tertunda
    pub async fn pending_count(&self) -> usize {
        let pending = self.pending_tasks.read().await;
        pending.len()
    }

    /// Get active assignments count
    /// Dapatkan jumlah penugasan aktif
    pub async fn assignment_count(&self) -> usize {
        let assignments = self.task_assignments.read().await;
        assignments.len()
    }

    /// Create P2P share task message
    /// Buat pesan berbagi task P2P
    pub fn create_share_task_message(
        &self,
        task: Task,
    ) -> Message {
        Message::P2PShareTask {
            task,
            requester_id: self.local_id.clone(),
        }
    }

    /// Create resource announcement message
    /// Buat pesan pengumuman resource
    pub async fn create_resource_announcement(&self) -> Result<Message> {
        let resources = self.resources.read().await;
        Ok(Message::ResourceAnnounce(resources.clone()))
    }

    /// Create peer discovery request message
    /// Buat pesan permintaan peer discovery
    pub fn create_discovery_request(&self) -> Message {
        Message::PeerDiscoveryRequest {
            requester_id: self.local_id.clone(),
            timestamp: chrono::Local::now().timestamp(),
        }
    }

    /// Handle peer discovery response
    /// Tangani respons peer discovery
    pub async fn handle_discovery_response(
        &self,
        responder_id: String,
        resources: ResourceAvailability,
    ) -> Result<()> {
        self.register_peer_resources(resources).await?;
        info!("[P2P] Peer discovery response handled: {}", responder_id);
        Ok(())
    }

    /// Handle P2P task response
    /// Tangani respons task P2P
    pub async fn handle_task_response(
        &self,
        task_id: String,
        accepted: bool,
        peer_id: String,
    ) -> Result<()> {
        if accepted {
            self.assign_to_peer(task_id.clone(), peer_id.clone()).await?;
            info!("[P2P] Task {} accepted by peer {}", task_id, peer_id);
        } else {
            warn!("[P2P] Task {} rejected by peer {}", task_id, peer_id);
        }
        Ok(())
    }

    /// Cleanup stale peer resources
    /// Bersihkan resource peer yang sudah lama
    pub async fn cleanup_stale_peers(&self, timeout_secs: i64) -> Result<()> {
        let mut peer_resources = self.peer_resources.write().await;
        let now = chrono::Local::now().timestamp();

        let stale: Vec<String> = peer_resources
            .iter()
            .filter(|(_, r)| (now - r.timestamp) > timeout_secs)
            .map(|(id, _)| id.clone())
            .collect();

        for peer_id in stale {
            info!("[P2P] Removing stale peer resources: {}", peer_id);
            peer_resources.remove(&peer_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distributor_creation() {
        let distributor = P2PDistributor::new(
            "peer1".to_string(),
            4,
            8192,
            false,
            4,
        );

        assert_eq!(distributor.pending_count().await, 0);
        assert_eq!(distributor.assignment_count().await, 0);
    }

    #[tokio::test]
    async fn test_enqueue_task() {
        let distributor = P2PDistributor::new(
            "peer1".to_string(),
            4,
            8192,
            false,
            4,
        );

        let task = Task::new("echo test".to_string());
        distributor.enqueue_p2p_task(task).await.unwrap();

        assert_eq!(distributor.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_peer_resource_registration() {
        let distributor = P2PDistributor::new(
            "peer1".to_string(),
            4,
            8192,
            false,
            4,
        );

        let peer_res = ResourceAvailability::new(
            "peer2".to_string(),
            8,
            16384,
            true,
            8,
        );

        distributor.register_peer_resources(peer_res).await.unwrap();
        assert_eq!(distributor.get_all_peers().await.len(), 1);
    }

    #[tokio::test]
    async fn test_find_best_peer() {
        let distributor = P2PDistributor::new(
            "peer1".to_string(),
            4,
            8192,
            false,
            4,
        );

        let peer_res = ResourceAvailability::new(
            "peer2".to_string(),
            8,
            16384,
            true,
            8,
        );

        distributor.register_peer_resources(peer_res).await.unwrap();

        let task = Task::new("test".to_string());
        let best_peer = distributor.find_best_peer(&task, 4, 8192, false).await;

        assert!(best_peer.is_some());
        assert_eq!(best_peer.unwrap(), "peer2");
    }
}

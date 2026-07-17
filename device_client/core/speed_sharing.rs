//! Speed Sharing Module
//! Handles secure bandwidth sharing between trusted FireSDN users
//! Integrates with Mesh Network and Routing Server
//! Author: ZatQasim (Mohamed Mohamed Diriye)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use crate::network_monitor::NetworkMetrics;
use crate::crypto_tools::encrypt_payload;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeerDevice {
    pub device_id: String,
    pub ip: String,
    pub port: u16,
    pub last_seen: Instant,
    pub shared_bandwidth_mbps: f32,
}

#[derive(Debug)]
pub struct SpeedSharingManager {
    peers: Arc<Mutex<HashMap<String, PeerDevice>>>,
    max_share_percentage: f32, // Maximum percentage of local bandwidth to share
}

impl SpeedSharingManager {
    pub fn new(max_share: f32) -> Self {
        SpeedSharingManager {
            peers: Arc::new(Mutex::new(HashMap::new())),
            max_share_percentage: max_share,
        }
    }

    /// Register a peer device for speed sharing
    pub fn register_peer(&self, device: PeerDevice) {
        let mut peers = self.peers.lock().unwrap();
        peers.insert(device.device_id.clone(), device);
    }

    /// Remove inactive peers
    pub fn cleanup_peers(&self) {
        let mut peers = self.peers.lock().unwrap();
        let now = Instant::now();
        peers.retain(|_, peer| now.duration_since(peer.last_seen) < Duration::from_secs(300));
    }

    /// Calculate bandwidth to share for each peer
    fn calculate_share(&self, local_metrics: &NetworkMetrics) -> HashMap<String, f32> {
        let peers = self.peers.lock().unwrap();
        let mut share_map = HashMap::new();
        let total_peers = peers.len() as f32;

        if total_peers == 0.0 { return share_map; }

        let available_bandwidth = local_metrics.download_mbps * self.max_share_percentage / 100.0;
        let per_peer_bandwidth = available_bandwidth / total_peers;

        for (device_id, _) in peers.iter() {
            share_map.insert(device_id.clone(), per_peer_bandwidth);
        }

        share_map
    }

    /// Send shared bandwidth data to peers
    pub fn distribute_bandwidth(&self, local_metrics: &NetworkMetrics) {
        let shares = self.calculate_share(local_metrics);
        let peers = self.peers.lock().unwrap();

        for (device_id, bw) in shares {
            if let Some(peer) = peers.get(&device_id) {
                let peer_clone = peer.clone();
                let encrypted_data = encrypt_payload(bw.to_le_bytes().to_vec());

                // Send asynchronously
                thread::spawn(move || {
                    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", peer_clone.ip, peer_clone.port)) {
                        let _ = stream.write_all(&encrypted_data);
                    }
                });
            }
        }
    }

    /// Listen for incoming shared bandwidth requests
    pub fn listen_for_shares(&self, port: u16) {
        let peers_arc = Arc::clone(&self.peers);
        thread::spawn(move || {
            let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind speed sharing port");

            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = vec![0u8; 1024];
                    if let Ok(size) = stream.read(&mut buffer) {
                        let decrypted = crate::crypto_tools::decrypt_payload(&buffer[..size]);
                        if let Ok(bw) = f32::from_le_bytes(decrypted.try_into().unwrap_or([0u8; 4])) {
                            let mut peers = peers_arc.lock().unwrap();
                            // Update peer bandwidth dynamically
                            // Here you could trigger further routing decisions
                            println!("Received shared bandwidth from peer: {} Mbps", bw);
                        }
                    }
                }
            }
        });
    }
}
use std::thread;
use std::time::Duration;

pub fn start_firewall() {
    thread::spawn(|| loop {
        // Placeholder firewall check
        println!("[Security Node] Firewall running...");
        thread::sleep(Duration::from_secs(5));
    });
}
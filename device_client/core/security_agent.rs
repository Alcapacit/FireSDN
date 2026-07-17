use std::thread;

pub fn start_security_agent() {
    thread::spawn(|| {
        loop {
            // Packet inspection / firewall hooks
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
}
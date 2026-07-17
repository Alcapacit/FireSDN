use std::time::{Instant, Duration};

pub fn measure_execution_time<F>(func: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    start.elapsed()
}

pub fn xor_encrypt_decrypt(data: &mut [u8], key: u8) {
    for byte in data.iter_mut() {
        *byte ^= key;
    }
}
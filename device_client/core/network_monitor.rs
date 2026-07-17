use std::fs;

pub fn collect_metrics() -> String {
    let data = fs::read_to_string("/proc/net/dev").unwrap_or_default();
    data
}
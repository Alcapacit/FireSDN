pub fn evaluate_context(metrics: &String) -> String {
    if metrics.len() < 500 {
        "low_latency".to_string()
    } else {
        "high_throughput".to_string()
    }
}
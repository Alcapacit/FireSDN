import json

METRICS_FILE = "../../Device_Client/cache/metrics_cache.json"

def predict_best_route():
    try:
        with open(METRICS_FILE, "r") as f:
            metrics = json.load(f)
        latency = metrics["device_metrics"]["latency_ms"]

        if latency < 50:
            return "local-node-1"
        elif latency < 100:
            return "regional-node-1"
        else:
            return "global-node-1"
    except Exception as e:
        return "fallback-node"
        
import json
import matplotlib.pyplot as plt

METRICS_PATH = "../../Device_Client/cache/metrics_cache.json"

def load_metrics():
    with open(METRICS_PATH, "r") as f:
        return json.load(f)["device_metrics"]

def generate_latency_chart():
    metrics = load_metrics()
    latency = metrics["latency_ms"]

    plt.figure()
    plt.bar(["Latency"], [latency])
    plt.ylabel("ms")
    plt.title("Network Latency")

    path = "latency.png"
    plt.savefig(path)
    plt.close()
    return path

def generate_bandwidth_chart():
    metrics = load_metrics()
    down = metrics["download_mbps"]
    up = metrics["upload_mbps"]

    plt.figure()
    plt.bar(["Download", "Upload"], [down, up])
    plt.ylabel("Mbps")
    plt.title("Bandwidth")

    path = "bandwidth.png"
    plt.savefig(path)
    plt.close()
    return path
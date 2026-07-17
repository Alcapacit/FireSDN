from flask import Flask, jsonify, send_file
import json
import time
from visualizations import generate_latency_chart, generate_bandwidth_chart

app = Flask(__name__)

METRICS_PATH = "../../Device_Client/cache/metrics_cache.json"
LOG_PATH = "../logs/dashboard_activity.log"

def log(message):
    timestamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    with open(LOG_PATH, "a") as f:
        f.write(f"[INFO] {timestamp} {message}\n")

@app.route("/api/metrics")
def get_metrics():
    with open(METRICS_PATH, "r") as f:
        data = json.load(f)
    log("Metrics requested")
    return jsonify(data)

@app.route("/api/charts/latency")
def latency_chart():
    path = generate_latency_chart()
    return send_file(path, mimetype="image/png")

@app.route("/api/charts/bandwidth")
def bandwidth_chart():
    path = generate_bandwidth_chart()
    return send_file(path, mimetype="image/png")

if __name__ == "__main__":
    log("Dashboard backend started")
    app.run(host="0.0.0.0", port=8080)
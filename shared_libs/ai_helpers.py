import math

def predict_latency_trend(latencies):
    if not latencies:
        return None
    avg = sum(latencies)/len(latencies)
    trend = (latencies[-1] - latencies[0])/len(latencies)
    return {"average": avg, "trend": trend}

def score_route(latency, bandwidth, security):
    """Compute a simple weighted score for routing selection"""
    return bandwidth * 0.5 + security * 50 - latency * 0.5
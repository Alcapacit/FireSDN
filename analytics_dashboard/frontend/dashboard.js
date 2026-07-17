async function loadMetrics() {
    const res = await fetch("/api/metrics");
    const data = await res.json();

    const m = data.device_metrics;
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/sw.js')
  .then(() => console.log('Service Worker Registered'));
}
    document.getElementById("metrics").innerHTML = `
        <p>Latency: ${m.latency_ms} ms</p>
        <p>Download: ${m.download_mbps} Mbps</p>
        <p>Upload: ${m.upload_mbps} Mbps</p>
        <p>Packet Loss: ${m.packet_loss_percent}%</p>
    `;
}

setInterval(loadMetrics, 3000);
loadMetrics();
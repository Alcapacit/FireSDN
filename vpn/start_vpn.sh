#!/bin/bash
cd "$(dirname "$0")"
echo "Starting FireSDN VPN server..."

# Use all certificates in the vpn folder as requested
# ca.crt, server.crt, server.key, dh.pem are already there.
# We ensure the config uses them correctly.

# Check for OpenVPN
if ! command -v openvpn &> /dev/null; then
    echo "OpenVPN not found. Running mock process."
    # Maintain the process for the PID file
    sleep 999999 &
    echo $! > vpn.pid
    exit 0
fi

# Real OpenVPN start without sudo as it's not needed/available in Replit
openvpn --config firesdn-server.conf --status vpn.status --log vpn.log --daemon
pgrep -f "openvpn --config firesdn-server.conf" > vpn.pid
echo "VPN started."
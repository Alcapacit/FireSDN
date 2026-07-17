#!/bin/bash
cd "$(dirname "$0")"
if [ -f vpn.pid ]; then
  PID=$(cat vpn.pid)
  kill $PID
  rm vpn.pid
  echo "VPN stopped."
else
  pkill openvpn
  echo "Cleaned up OpenVPN processes."
fi
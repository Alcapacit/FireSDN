#!/bin/bash
timestamp=$(date +%Y%m%d_%H%M%S)
backup_folder="backup_$timestamp"
mkdir $backup_folder
cp *.conf *.crt *.key *.pem *.ovpn $backup_folder/
echo "Backup completed in folder $backup_folder"
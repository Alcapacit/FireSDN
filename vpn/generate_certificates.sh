#!/bin/bash
set -e

# Create CA
openssl genrsa -out ca.key 4096
openssl req -x509 -new -nodes -key ca.key -sha256 -days 3650 -out ca.crt -subj "/C=SO/ST=Somalia/L=Hargeisa/O=FireSDN VPN/OU=CA/CN=FireSDN CA"

# Server Key & Certificate
openssl genrsa -out server.key 4096
openssl req -new -key server.key -out server.csr -subj "/C=SO/ST=Somalia/L=Hargeisa/O=FireSDN VPN/OU=Server/CN=firesdn-server"
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 3650 -sha256

# Client Key & Certificate
openssl genrsa -out client1.key 4096
openssl req -new -key client1.key -out client1.csr -subj "/C=SO/ST=Somalia/L=Hargeisa/O=FireSDN VPN/OU=Client/CN=firesdn-client"
openssl x509 -req -in client1.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client1.crt -days 3650 -sha256

# Diffie-Hellman
openssl dhparam -out dh.pem 2048

# Cleanup
rm server.csr client1.csr

echo "All certificates and keys generated!"
#!/bin/sh

cat /etc/hosts

ip addr

cat /etc/resolv.conf

host quicknet-server
host quicknet_server

ping -c1 quicknet-server

echo "Preparing for launch..."
sleep 5

/quicknet_client


#!/bin/sh

set -x

SERVER_NS=server

CLIENT_NS=client



sudo ip netns del $SERVER_NS
sudo ip netns del $CLIENT_NS

sudo ip link del br0
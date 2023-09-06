#!/bin/sh

set -x

SERVER_NS=server
SERVER_IF=mv1

CLIENT_NS=client
CLIENT_IF=mv2

# sudo ip link add $SERVER_IF type veth peer name $CLIENT_IF



sudo ip netns add $SERVER_NS
# sudo ip link set $SERVER_IF netns $SERVER_NS

# sudo ip netns exec $SERVER_NS ip addr add 172.17.42.11/24 dev $SERVER_IF
# sudo ip netns exec $SERVER_NS ip link set dev $SERVER_IF up


sudo ip netns add $CLIENT_NS
# sudo ip link set $CLIENT_IF netns $CLIENT_NS

# sudo ip netns exec $CLIENT_NS ip addr add 172.17.42.12/24 dev $CLIENT_IF
# sudo ip netns exec $CLIENT_NS ip link set dev $CLIENT_IF up

# sudo ip netns exec $SERVER_NS ip link list
# sudo ip netns exec $CLIENT_NS link list


sudo ip link add name br0 type bridge
sudo ip link set br0 up


# create the macvlan link attaching it to the parent host eth0
sudo ip link add mv1 link br0 type macvlan mode bridge
sudo ip link add mv2 link br0 type macvlan mode bridge

# move the new interface mv1/mv2 to the new namespace
sudo ip link set mv1 netns $SERVER_NS
sudo ip link set mv2 netns $CLIENT_NS

# bring the two interfaces up
sudo ip netns exec $SERVER_NS ip link set dev $SERVER_IF up
sudo ip netns exec $CLIENT_NS ip link set dev $CLIENT_IF up

# set ip addresses
sudo ip netns exec $SERVER_NS ip add add 172.17.42.11/24 dev $SERVER_IF
sudo ip netns exec $SERVER_NS ip link set dev $SERVER_IF up

sudo ip netns exec $CLIENT_NS ip add add 172.17.42.12/24 dev $CLIENT_IF
sudo ip netns exec $CLIENT_NS ip link set dev $CLIENT_IF up

# ping from one ns to another
sudo ip netns exec $CLIENT_NS ping -c4 172.17.42.11

# Performance Test
sudo ip netns exec $SERVER_NS iperf3 -s -p 55555 &
sudo ip netns exec $CLIENT_NS iperf3 -c 172.17.42.11 -p 55555 -t 5

sudo ip netns exec $SERVER_NS xterm &

sudo ip netns exec $CLIENT_NS xterm &

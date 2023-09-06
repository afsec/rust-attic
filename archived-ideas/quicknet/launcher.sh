#!/bin/bash

while true;do
    (sleep 1 ; ./client/dist/client) &
    (sleep 1 ; ./client/dist/client) &
    (sleep 2 ; ./client/dist/client) &
    (sleep 2 ; ./client/dist/client) &
    echo "Preparing next wave..."
    sleep 3
done

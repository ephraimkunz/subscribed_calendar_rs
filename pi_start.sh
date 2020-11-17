#!/bin/bash
# To be run on the Raspberry Pi to start the server.

if [[ $UID != 0 ]]; then
    echo "Please run this script with sudo:"
    echo "sudo $0 $*"
    exit 1
fi

./target/release/subscribed_calendar_rs >log.txt 2>&1 &

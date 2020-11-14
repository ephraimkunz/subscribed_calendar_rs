#!/bin/bash
# To be run on the Raspberry Pi to build and start the server.

if [[ $UID != 0 ]]; then
    echo "Please run this script with sudo:"
    echo "sudo $0 $*"
    exit 1
fi

cargo build --release
ROCKET_PORT=80 nohup ./target/subscribed_calendar_rs >log.txt 2>&1 &
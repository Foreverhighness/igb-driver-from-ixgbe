#!/bin/bash

set -x

mkdir test_temp
cd test_temp

git clone --depth=1 https://github.com/Foreverhighness/igb-driver-from-ixgbe.git igb-driver
git clone --depth=1 https://github.com/Foreverhighness/arceos.git

make -C arceos A=examples/httpserver PLATFORM=aarch64-qemu-virt LOG=info SMP=2 NET=y FEATURES=driver-igb NET_DEV=user run &

let retry=10
sleep 3
while ! curl localhost:5555; do
    if [[ "$retry" -le "0" ]]; then
        break;
    fi
    sleep 1
done

pkill qemu
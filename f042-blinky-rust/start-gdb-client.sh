#!/bin/bash
./build.sh
#arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/blinky -ex "target remote localhost:3333"
arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/blinky -x gdb-client-config.gdb

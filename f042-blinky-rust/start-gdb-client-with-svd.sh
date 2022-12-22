#!/bin/bash
./build.sh
arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/blinky -x gdb-client-config-with-svd.gdb

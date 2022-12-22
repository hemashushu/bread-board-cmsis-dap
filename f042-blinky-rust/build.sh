#!/bin/bash
cargo clean
cargo build

# convert ELF to BIN
arm-none-eabi-objcopy -O binary -S \
    target/thumbv6m-none-eabi/debug/blinky \
    target/thumbv6m-none-eabi/debug/blinky.bin
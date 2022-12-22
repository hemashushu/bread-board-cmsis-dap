#!/bin/bash
cargo clean

# ## flash by `cargo-flash` (https://github.com/probe-rs/probe-rs)
#
# 1. install `cargo-flash`
#    `$ cargo install cargo-flash`
# 2. check probe connection
#    `$ cargo flash --list-probes`
# 3. chip name can be found by
#    `$ cargo flash --list-chips`
# 4. flash
#    e.g.
#    `$ cargo flash --chip STM32F042F6Px`
#    or flash with _release_ profile
#    `$ cargo flash --release --chip STM32F042F6Px`
#    or flash an _example_
#    `$ cargo flash --example <your_example> --chip STM32F042F6PxS`

cargo flash --chip STM32F042F6Px
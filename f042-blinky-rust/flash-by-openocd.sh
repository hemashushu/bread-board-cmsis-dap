#!/bin/bash
./build.sh

# ## flash by OpenOCD
#
# with full script path specified:
# openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -s "/usr/share/openocd/scripts" -c "program target/thumbv6m-none-eabi/debug/blinky verify reset exit"
#
# flash BIN:
# openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -c "program target/thumbv6m-none-eabi/debug/blinky.bin verify reset exit 0x08000000"
#
# flash ELF:
# openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -c "program target/thumbv6m-none-eabi/debug/blinky verify reset exit"

openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -c "program target/thumbv6m-none-eabi/debug/blinky verify reset exit"

#!/bin/bash
cd src
./build-f042.sh
# openocd default scripts location "/usr/share/openocd/scripts"
openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -c "program STM32F042-BREADBOARD.elf verify reset exit"
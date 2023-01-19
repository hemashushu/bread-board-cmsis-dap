#!/bin/bash
./build.sh
# openocd default scripts location "/usr/share/openocd/scripts"
openocd -f interface/cmsis-dap.cfg  -f target/stm32f0x.cfg -c "program main.elf verify reset exit"
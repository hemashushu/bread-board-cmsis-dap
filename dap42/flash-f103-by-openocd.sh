#!/bin/bash
cd src
./build-f103.sh
# openocd default scripts location "/usr/share/openocd/scripts"
openocd -f interface/cmsis-dap.cfg  -f target/stm32f1x.cfg -c "program STM32F103-BLUEPILL.elf verify reset exit"
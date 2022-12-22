#!/bin/bash
cd src
./build-f042.sh
# https://dfu-util.sourceforge.net/
dfu-util -a 0 -s 0x08000000:leave -D STM32F042-BREADBOARD.bin
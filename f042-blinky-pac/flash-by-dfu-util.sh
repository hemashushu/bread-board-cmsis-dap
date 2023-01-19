#!/bin/bash
./build.sh
# https://dfu-util.sourceforge.net/
dfu-util -a 0 -s 0x08000000:leave -D main.bin
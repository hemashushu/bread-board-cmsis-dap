#!/bin/bash
cd src
make clean
make STM32F042-BREADBOARD.bin
arm-none-eabi-size STM32F042-BREADBOARD.elf

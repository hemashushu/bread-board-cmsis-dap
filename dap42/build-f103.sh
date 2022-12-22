#!/bin/bash
cd src
make clean
make STM32F103-BLUEPILL.bin
arm-none-eabi-size STM32F103-BLUEPILL.elf

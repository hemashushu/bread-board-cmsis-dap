#!/bin/bash

rm main.elf
rm main.bin

# FPU options
#
# CFLAGS += -mfloat-abi=soft # No FP
# CFLAGS += -mfloat-abi=softfp -mfpu=fpv4-sp-d16 # Soft FP
# CFLAGS += -mfloat-abi=hard -mfpu=fpv4-sp-d16 # Hard FP
#
# start files and standard libraries
#
# LDFLAGS += -nostartfiles # dont use standard start files
# LDFLAGS += -nodefaultlibs # dont use standard libraries
# LDFLAGS += -nostdlib # dont use startup or default libs

arm-none-eabi-gcc \
    -mcpu=cortex-m0 \
    -mthumb \
    -Wall -g \
    --specs=nosys.specs \
    -nostartfiles \
    -Wl,-T,linker.ld \
    -o main.elf \
    main.c

arm-none-eabi-size main.elf
arm-none-eabi-objcopy -O binary -S main.elf main.bin

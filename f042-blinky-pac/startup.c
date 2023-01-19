/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#include <stdint.h>

extern void NMI_Handler();
extern void HardFault_Handler();
extern void SVC_Handler();
extern void PendSV_Handler();
extern void SysTick_Handler();

extern void main();

// the true startup code
// keyword `naked` indicates this function no function prologue.
__attribute__((naked, noreturn)) void Reset_Handler()
{
    // set initial stack pointer
    //
    // note::
    // since the stack point is already included in the vector_table (in the first entry),
    // this statement is not necessary.
    //
    // asm("ldr sp, = _estack");

    // memset .bss to zero, and copy .data section to RAM region
    extern long _sbss, _ebss, _sdata, _edata, _sidata;

    // initialize `BSS`
    // set all bytes within `.bss` to `0`
    for (long *mem_addr = &_sbss; mem_addr < &_ebss; mem_addr++)
    {
        *mem_addr = 0;
    }

    // initialize `Data`
    // copy the content of `.data` from `flash` to `RAM`
    for (long *mem_addr = &_sdata, *flash_addr = &_sidata; mem_addr < &_edata;)
    {
        *mem_addr++ = *flash_addr++;
    }

    // call user's main function
    main();

    // infinite loop in the case if main() returns
    for (;;)
    {
        (void)0;
    }
}

// PM0215 2.3.4 Vector table
//
// vector table entry list and item name from
// `STM32Cube/Repository/STM32Cube_FW_F0_V1.11.3/Drivers/CMSIS/Device/ST/STM32F0xx/Source/Templates/gcc/startup_stm32f031x6.s`
// `STM32Cube/Repository/STM32Cube_FW_F0_V1.11.3/Drivers/CMSIS/Device/ST/STM32F0xx/Source/Templates/gcc/startup_stm32f042x6.s`
__attribute__((section(".vector_table.exceptions"))) void (*Exceptions[16])() = {
    // &_estack,        // idx: 0 the initial stack pointer
    Reset_Handler,            // idx: 1 the address of the entry function
    NMI_Handler,              // idx: 2
    HardFault_Handler,        // idx: 3
    0,                        // idx: 4
    0,                        // idx: 5
    0,                        // idx: 6
    0,                        // idx: 7
    0,                        // idx: 8
    0,                        // idx: 9
    0,                        // idx: 10
    SVC_Handler,              // idx: 11
    0,                        // idx: 12
    0,                        // idx: 13
    PendSV_Handler,           // idx: 14
    SysTick_Handler,          // idx: 15
};

__attribute__((naked, noreturn)) void Default_Handler()
{
    for (;;)
    {
        (void)0;
    }
}
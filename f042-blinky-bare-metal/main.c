/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#include <stdint.h>

// RM0091 8.4 GPIO registers
typedef struct
{
    volatile uint32_t MODER, OTYPER, OSPEEDR, PUPDR, IDR, ODR, BSRR, LCKR, AFR[2], BRR;
} GPIO_TypeDef;

// RM0091 6.4 RCC registers
typedef struct
{
    volatile uint32_t CR, CFGR, CIR, APB2RSTR, APB1RSTR, AHBENR, APB2ENR, APB1ENR, BDCR,
        CSR, AHBRSTR, CFGR2, CFGR3, CR2;
} RCC_TypeDef;

// RM0091 2.2.2 Memory map and register boundary addresses
#define GPIOB ((GPIO_TypeDef *)0x48000400UL)
#define RCC ((RCC_TypeDef *)0x40021000UL)

// RM0091 6.4.6 AHB peripheral clock enable register (RCC_AHBENR)
#define RCC_AHBENR_GPIOBEN (0x1UL << 18U)

void blinky()
{
    // LED pin is `PB1`

    // enable the GPIOB peripheral clock
    // RM0091 6.4.6 AHB peripheral clock enable register (RCC_AHBENR)
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;

    // initialize the LED pin
    uint32_t led_pin_number = 1;

    const uint32_t GPIO_MODER_Msk = 0x3;   // 0b11
    const uint32_t GPIO_OSPEEDR_Msk = 0x3; // 0b11

    // RM0091 8.4.12 GPIO register map
    GPIOB->MODER &= ~(GPIO_MODER_Msk << (led_pin_number * 2));     // clear bits
    GPIOB->MODER |= (0x1 << (led_pin_number * 2));                 // set mode to `output`
    GPIOB->OSPEEDR &= ~(GPIO_OSPEEDR_Msk << (led_pin_number * 2)); // clear bits, default speed to `low`
    GPIOB->OTYPER &= ~(1 << led_pin_number);                       // clear bits, default output type to `push-pull`

    while (1)
    {
        // RM0091 8.4.6 GPIO port output data register (GPIOx_ODR)
        GPIOB->ODR &= ~(1 << led_pin_number); // set `0` to turn on LED
        for (int i = 0; i < 20000; i++)
        {
        }

        GPIOB->ODR |= (1 << led_pin_number); // set `1` to turn off LED
        for (int i = 0; i < 400000; i++)
        {
        }
    }
}

int main()
{
    blinky();
}

extern void NMI_Handler();
extern void HardFault_Handler();

// the true startup code
// keyword `naked` indicates this function no function prologue.
__attribute__((naked, noreturn)) void Reset_Handler()
{
    // note:
    // memset .bss to zero, and copy .data section to RAM region here

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
__attribute__((section(".vector_table.exceptions"))) void (*Exceptions[4])() = {
    // &_estack,        // idx: 0 the initial stack pointer
    Reset_Handler,     // idx: 1 the address of the entry function
    NMI_Handler,       // idx: 2
    HardFault_Handler, // idx: 3
};

__attribute__((naked, noreturn)) void Default_Handler()
{
    for (;;)
    {
        (void)0;
    }
}
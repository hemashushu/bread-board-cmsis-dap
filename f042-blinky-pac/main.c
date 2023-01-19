/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#include <stdint.h>

#include "stm32f042x6.h"

void blinky()
{
    // LED pin is `PA5`

    // enable the GPIOA peripheral clock
    RCC->AHBENR |= RCC_AHBENR_GPIOAEN;

    // initialize the LED pin
    uint32_t led_pin_number = 5;

    const uint32_t GPIO_MODER_Msk = 0x3;   // 0b11
    const uint32_t GPIO_OSPEEDR_Msk = 0x3; // 0b11

    GPIOA->MODER &= ~(GPIO_MODER_Msk << (led_pin_number * 2));     // clear bits
    GPIOA->MODER |= (0x1 << (led_pin_number * 2));                 // set mode to `output`
    GPIOA->OSPEEDR &= ~(GPIO_OSPEEDR_Msk << (led_pin_number * 2)); // clear bits, default speed to `low`
    GPIOA->OTYPER &= ~(1 << led_pin_number);                       // clear bits, default output type to `push-pull`

    while (1)
    {
        GPIOA->ODR &= ~(1 << led_pin_number); // set `0` to turn on LED
        for (int i = 0; i < 100000; i++)
        {
        }

        GPIOA->ODR |= (1 << led_pin_number); // set `1` to turn off LED
        for (int i = 0; i < 100000; i++)
        {
        }
    }
}

int main()
{
    blinky();
}

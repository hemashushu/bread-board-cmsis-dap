/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#include <stdint.h>

#include "cmsis/stm32f042x6.h"

void blinky()
{
    // LED pin is `PB1`

    // enable the GPIOB peripheral clock
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;

    // initialize the LED pin
    uint32_t led_pin_number = 1;

    GPIOB->MODER &= ~(0x3 << (led_pin_number * 2));   // clear bits
    GPIOB->MODER |= (0x1 << (led_pin_number * 2));    // set bits to 0x01, set mode to `output`
    GPIOB->OSPEEDR &= ~(0x3 << (led_pin_number * 2)); // clear bits, set speed to `low`
    GPIOB->OTYPER &= ~(1 << led_pin_number);          // clear bits, set output type to `push-pull`

    while (1)
    {
        GPIOB->ODR &= ~(1 << led_pin_number); // set `0` to turn on LED
        for (int i = 0; i < 100000; i++)
        {
        }

        GPIOB->ODR |= (1 << led_pin_number); // set `1` to turn off LED
        for (int i = 0; i < 100000; i++)
        {
        }
    }
}

int main()
{
    blinky();
}

// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

#[macro_use]
mod utils;
mod common;
mod config;
mod startup;

mod register_gpio;
mod register_rcc;

use core::panic::PanicInfo;

use config::BUILTIN_LED_PIN;
use register_gpio::{
    get_GPIO, GPIO_MODER_Msk, GPIO_OSPEEDR_Msk, GPIO_MODER, GPIO_OSPEEDR, GPIO_OTYPER,
};
use register_rcc::{get_RCC, RCC_AHBENR};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        //
    }
}

#[no_mangle]
#[export_name = "main"]
pub extern "C" fn bare_main() -> ! {
    blinky();

    loop {
        //
    }
}

fn blinky() {
    let RCC = unsafe { &mut *get_RCC() };
    let GPIOB = unsafe { &mut *get_GPIO(BUILTIN_LED_PIN.port) };

    // enable the GPIOB peripheral clock
    // RM0091 6.4.6 AHB peripheral clock enable register (RCC_AHBENR)
    RCC.AHBENR |= RCC_AHBENR::GPIOBEN as u32;

    // initialize the LED pin
    let led_pin_number = BUILTIN_LED_PIN.number;
    GPIOB.MODER &= !(GPIO_MODER_Msk << (led_pin_number * 2)); // clear bits
    GPIOB.MODER |= (GPIO_MODER::OUTPUT as u32) << (led_pin_number * 2); // set mode to `output`
    GPIOB.OSPEEDR &= !(GPIO_OSPEEDR_Msk << (led_pin_number * 2)); // clear bits
    GPIOB.OSPEEDR |= (GPIO_OSPEEDR::LOW as u32) << (led_pin_number * 2); // set speed to `low`
    GPIOB.OTYPER &= !(1 << led_pin_number); // clear bits
    GPIOB.OTYPER |= (GPIO_OTYPER::PUSH_PULL as u32) << led_pin_number; // set output type to `push-pull`

    loop {
        GPIOB.ODR &= !(1 << led_pin_number); // set `0` to turn on LED
        for _ in 0..2000 {}

        GPIOB.ODR |= 1 << led_pin_number; // set `1` to turn off LED
        for _ in 0..20000 {}
    }
}

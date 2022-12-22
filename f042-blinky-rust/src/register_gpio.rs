// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::common::Port;

// RM0091 8.4 GPIO registers
#[repr(C)]
pub struct GPIO {
    pub MODER: u32,
    pub OTYPER: u32,
    pub OSPEEDR: u32,
    pub PUPDR: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSRR: u32,
    pub LCKR: u32,
    pub AFR: [u32; 2],
    pub BRR: u32,
}

// RM0091 8.4.1 GPIO port mode register (GPIOx_MODER)
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GPIO_MODER {
    INPUT,
    OUTPUT,
    ALT,
    ANALOG,
}

// RM0091 8.4.2 GPIO port output type register (GPIOx_OTYPER)
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GPIO_OTYPER {
    PUSH_PULL,
    OPEN_DRAIN,
}

// RM0091 8.4.3 GPIO port output speed register (GPIOx_OSPEEDR)
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GPIO_OSPEEDR {
    LOW,
    MEDIUM,
    HIGH,
}

// RM0091 8.4.4 GPIO port pull-up/pull-down register (GPIOx_PUPDR)
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GPIO_PUPDR {
    NONE,
    PULL_UP,
    PULL_DOWN,
}

pub const GPIO_MODER_Msk :u32  = 0b11;
pub const GPIO_OSPEEDR_Msk: u32 = 0b11;

// RM0091 2.2.2 Table 1. STM32F0xx peripheral register boundary addresses (continued)
//
// GPIOF 0x4800_1400
// GPIOE 0x4800_1000
// GPIOD 0x4800_0C00
// GPIOC 0x4800_0800
// GPIOB 0x4800_0400
// GPIOA 0x4800_0000
pub fn get_GPIO(port: Port) -> *mut GPIO {
    let addr: u32 = 0x4800_0000 + 0x400 * port as u32;
    addr as *mut GPIO
}

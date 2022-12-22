// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// register reference
// - `STM32Cube/Repository/STM32Cube_FW_F0_V1.11.3/Drivers/CMSIS/Include/`
//   - `core_cm0.h`
// `STM32Cube/Repository/STM32Cube_FW_F0_V1.11.3/Drivers/CMSIS/Device/ST/STM32F0xx/Include/`
//   - `stm32f042x6.h`

// RM0091 6.4 RCC registers
#[repr(C)]
pub struct RCC {
    pub CR: u32,
    pub CFGR: u32,
    pub CIR: u32,
    pub APB2RSTR: u32,
    pub APB1RSTR: u32,
    pub AHBENR: u32,
    pub APB2ENR: u32,
    pub APB1ENR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub AHBRSTR: u32,
    pub CFGR2: u32,
    pub CFGR3: u32,
    pub CR2: u32,
}

// RM0091 6.4.6 AHB peripheral clock enable register (RCC_AHBENR)
#[repr(u32)]
pub enum RCC_AHBENR {
    DMAEN = bit_to_u32!(0),
    SRAMEN = bit_to_u32!(2),
    FLITFEN = bit_to_u32!(4),
    CRCEN = bit_to_u32!(6),
    GPIOAEN = bit_to_u32!(17),
    GPIOBEN = bit_to_u32!(18),
    GPIOCEN = bit_to_u32!(19),
    GPIODEN = bit_to_u32!(20),
    GPIOEEN = bit_to_u32!(21),
    GPIOFEN = bit_to_u32!(22),
    TSCEN = bit_to_u32!(24),
}

// RM0091 2.2.2 Memory map and register boundary addresses
pub fn get_RCC() -> *mut RCC {
    let addr: u32 = 0x4002_1000;
    addr as *mut RCC
}

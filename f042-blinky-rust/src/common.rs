// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub static mut SYSCLK: u32 = 48_000_000; //
pub static mut HCLK: u32 = 48_000_000; // for AHB bus, core, memory and DMA, max 48MHz
pub static mut PCLK: u32 = 48_000_000; // for APB1, max 48MHz

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Port {
    A = 0, // the `enum` entry value starts from 0 by default
    B,
    C,
    D,
    E,
    F,
}

pub struct Pin {
    pub port: Port,
    pub number: u8,
}

impl Pin {
    pub const fn new(port: Port, number: u8) -> Self {
        Self { port, number }
    }
}

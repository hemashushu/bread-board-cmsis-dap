// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ptr;

// PM0214 2.3.2 Exception types
// Table 17. Properties of the different exception types
extern "C" {
    /**
     * A NonMaskable Interrupt (NMI) can be signalled by a peripheral or
     * triggered by software. This is the highest priority exception other than
     * reset. It is permanently enabled and has a fixed priority of -2. NMIs
     * cannot be:
     *
     * - Masked or prevented from activation by any other exception
     * - Preempted by any exception other than Reset.
     */
    fn NMI_Handler();

    /**
     * A hard fault is an exception that occurs because of an error during
     * exception processing, or because an exception cannot be managed by
     * any other exception mechanism. Hard faults have a fixed priority of -1,
     * meaning they have higher priority than any exception with configurable
     * priority.
     */
    fn HardFault_Handler();

    /**
     * A supervisor call (SVC) is an exception that is triggered by the SVC
     * instruction. In an OS environment, applications can use SVC
     * instructions to access OS kernel functions and device drivers.
     */
    fn SVC_Handler();

    /**
     * PendSV is an interrupt-driven request for system-level service. In an
     * OS environment, use PendSV for context switching when no other
     * exception is active.
     */
    fn PendSV_Handler();

    /**
     * A SysTick exception is an exception the system timer generates when
     * it reaches zero. Software can also generate a SysTick exception. In an
     * OS environment, the processor can use this exception as system tick.
     */
    fn SysTick_Handler();
}

extern "C" {
    fn main() -> !;
}

#[no_mangle]
pub extern "C" fn Reset_Handler() -> ! {
    // These symbols come from `linker.ld`
    //
    // note:
    // the value of the variable (e.g. `_sbss`) following the `extern "C" { static ...` is the
    // data of the address, but what we need is the address itself.
    // so use `&_sbss` to get the address value.
    // e.g.
    // the `_sbss` value is the memory data from address `0x2000_0100`.
    // the `&_sbss` value is `0x2000_0100`.
    extern "C" {
        static mut _sbss: u8; // Start of .bss section
        static _ebss: u8; // End of .bss section
        static mut _sdata: u8; // Start of .data section
        static _edata: u8; // End of .data section
        static _sidata: u8; // Start of .rodata section
        static _estack: u8; // Stack bottom
        static _heap_start: u8; // Heap start
    }

    // set initial stack pointer
    //
    // note::
    // since the stack point is already included in the vector_table (in the first entry),
    // this statement is not necessary.
    //
    // unsafe {
    //     asm!("ldr sp, = _estack",);
    // }

    // initialize `BSS`
    // set all bytes within `.bss` to `0`
    //
    // note::
    // `write_volatile` means volatile write of a memory
    //
    // unsafe {
    //     (&_sbss as *const u8 as u32..&_ebss as *const u8 as u32)
    //         .for_each(|dest_addr| (dest_addr as *mut u8).write_volatile(0))
    // }
    //
    unsafe {
        let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
        ptr::write_bytes(&mut _sbss as *mut u8, 0, count);
    }

    // initialize `Data`
    // copy the content of `.data` from `flash` to `RAM`
    //
    // unsafe {
    //     let data_source_addr = &_sidata as *const u8 as u32;
    //     (&_sdata as *const u8 as u32..&_edata as *const u8 as u32)
    //         .enumerate()
    //         .for_each(|(index, dest_addr)| {
    //             (dest_addr as *mut u8)
    //                 .write_volatile(*((index as u32 + data_source_addr) as *const u8))
    //         })
    // }

    unsafe {
        let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
        ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
    }

    // Call user's main function
    unsafe { main() }
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static Reset_Vector: extern "C" fn() -> ! = Reset_Handler;

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static Exceptions: [Vector; 14] = [
    Vector {
        handler: NMI_Handler,
    }, // idx 2
    Vector {
        handler: HardFault_Handler,
    }, // idx 3
    Vector { reserved: 0 }, // idx 4
    Vector { reserved: 0 }, // idx 5
    Vector { reserved: 0 }, // idx 6
    Vector { reserved: 0 }, // idx 7
    Vector { reserved: 0 }, // idx 8
    Vector { reserved: 0 }, // idx 9
    Vector { reserved: 0 }, // idx 10
    Vector {
        handler: SVC_Handler,
    }, // idx 11
    Vector { reserved: 0 }, // idx 12
    Vector { reserved: 0 }, // idx 13
    Vector {
        handler: PendSV_Handler,
    }, // idx 14
    Vector {
        handler: SysTick_Handler,
    }, // idx 15

    // IRQ starts here

    //  An interrupt, or IRQ, is an exception signalled by a peripheral, or
    //  generated by a software request. All interrupts are asynchronous to
    //  instruction execution. In the system, peripherals use interrupts to
    //  communicate with the processor.
];

#[no_mangle]
pub extern "C" fn Default_Handler() -> ! {
    loop {}
}

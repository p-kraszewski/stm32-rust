#![feature(asm,lang_items, core_intrinsics)]
#![feature(trace_macros)]
// We won't use the usual `main` function. We are going to use a different
// "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like
// threads and files and those are not available on this platform.
#![no_std]

#![allow(non_snake_case)]

#[macro_use]
extern crate stm32;

use stm32::f437::*;
use stm32::prelude::breakpoint as bp;

vectors!{
    Some(bp), Some(bp), Some(bp), Some(bp), Some(bp), None,
    None, None, None, Some(bp), None, None, Some(bp), Some(bp)
         }

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    power_on_gpioe();
    put_pe9_in_output_mode();
    set_pe9_high();
    set_pe9_low();

    loop {}
}

fn power_on_gpioe() {
    // /// Start address of the RCC register block
    // const RCC: u32 = 0x4002_1000;
    //
    // /// Offset address of the AHBENR register
    // const RCC_AHBENR: u32 = 0x14;
    //
    // /// IOPCEN bit mask
    //    const RCC_AHBENR_IOPEEN: u32 = 1 << 21;
    //    // unsafe {
    //    //     // Pointer to the AHBENR register
    //    //     let ahbenr = (RCC + RCC_AHBENR) as *mut u32;
    //    //
    //    //     // IOPECN = 1
    //    //     *ahbenr |= RCC_AHBENR_IOPEEN;
    //    // }
    //    unsafe {
    //        AHB1[(0x1000 + 0x0014) / 4] |= RCC_AHBENR_IOPEEN;
    //    }
    let n = io!{get SPEED from GPIOE};
    io!{set SPEED in GPIOE to n+5};
}

fn put_pe9_in_output_mode() {
    io!{with GPIOE do setmode(GPIO_PIN::PIN09, GPIO_MODE::Output)};
}

fn set_pe9_high() {
    io!{with GPIOE do bset(0b0000001000000000) };
}

fn set_pe9_low() {
    io!{with GPIOE do breset(0b0000001000000000) };
}

// Finally, we need to define the panic_fmt "lang item", which is just a
// function. This specifies what the program should do when a `panic!` occurs.
// Our program won't panic, so we can leave the function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}

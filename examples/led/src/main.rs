// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

#[macro_use]
extern crate alloc;

#[global_allocator]
static ALLOCATOR: nxdk_rs::alloc::XboxKernelAlloc = nxdk_rs::alloc::XboxKernelAlloc {};

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    nxdk_rs::hal::led::xset_custom_led(
        nxdk_rs::hal::led::LEDColor::Off,
        nxdk_rs::hal::led::LEDColor::Green,
        nxdk_rs::hal::led::LEDColor::Red,
        nxdk_rs::hal::led::LEDColor::Orange,
    );

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

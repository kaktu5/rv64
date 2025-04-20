#![no_std]
#![no_main]

mod boot;
mod drivers;

use crate::drivers::virt_uart;
use core::{
    // arch::asm,
    panic::PanicInfo,
    ptr::write_volatile,
};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    println!("Hello, world!");
    panic!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
    // loop {
    //     unsafe { asm!("wfi") }
    // }
    unsafe {
        write_volatile(0x100000 as *mut u32, 0x5555);
    }
    unreachable!();
}

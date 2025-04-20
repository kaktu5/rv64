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
    loop {
        if let Some(byte) = virt_uart::get() {
            if byte == 3 {
                break;
            }
            match byte {
                3 => break,
                13 => print!("\n"),
                _ => print!("{}", byte as char),
            }
        }
    }
    panic!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[PANIC] [{} {}:{}]: {}",
            location.file(),
            location.line(),
            location.column(),
            info.message()
        );
    };
    // loop {
    //     unsafe { asm!("wfi") }
    // }
    unsafe {
        write_volatile(0x100000 as *mut u32, 0x5555);
    }
    unreachable!();
}

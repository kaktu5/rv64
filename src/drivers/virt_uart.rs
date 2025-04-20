use core::{
    fmt::{Arguments, Result, Write},
    ptr::write_volatile,
};

const UART_TX: *mut u8 = 0x10000000 as *mut u8;

struct Uart;

impl Write for Uart {
    fn write_str(&mut self, str: &str) -> Result {
        str.bytes()
            .for_each(|c| unsafe { write_volatile(UART_TX, c) });
        Ok(())
    }
}

pub fn print(arg: Arguments) {
    Uart.write_fmt(arg).expect("failed to send by UART");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::virt_uart::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($arg:expr) => (print!(concat!($arg, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

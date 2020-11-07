use core::fmt::Write;

pub struct UartConsole;

unsafe fn uart_console_write(byte: u8) {
    let uart = 0x10000000 as *mut u8;
    *uart = byte;
}

impl UartConsole {
    // This function should be unsafe, since it allows creation of multiple
    // unsynchronized instances of an I/O device.
    // However, since this is a proof-of-concept OS, let's keep it simple :)
    pub fn new() -> UartConsole {
        UartConsole
    }
}

impl Write for UartConsole {
    fn write_str(&mut self, msg: &str) -> Result<(), core::fmt::Error> {
        for byte in msg.bytes() {
            unsafe {
                uart_console_write(byte);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!(crate::uart::UartConsole::new(), $($args)+).unwrap()
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        crate::print!("\n")
    });
    ($fmt:expr) => ({
        crate::print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        crate::print!(concat!($fmt, "\n"), $($args)+)
    });
}

use core::ptr::{read_volatile, write_volatile};
use core::fmt::{Write, Result};

pub struct Uart {
    thr: *mut u8,
    lsr: *mut u8,
}

pub const UART0: Uart = Uart {
    thr: 0x01D0C000 as *mut u8,
    lsr: 0x01D0C014 as *mut u8,
};

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for ch in s.bytes() {
            unsafe {
                while (read_volatile(self.lsr) & (1 << 5)) == 0 {}
                write_volatile(self.thr, ch);
            }
        }
        return Ok(());
    }
}

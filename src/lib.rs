#![feature(asm)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate rlibc;
extern crate compiler_builtins;

use core::ptr::{read_volatile, write_volatile};

//const UART_THR: *mut u8 = 0x101f1000 as *mut u8;
const UART_THR: *mut u8 = 0x01D0C000 as *mut u8;
const UART_LSR: *mut u8 = 0x01D0C014 as *mut u8;

#[no_mangle]
pub extern "C" fn rust_main() {
    putchar('v');
    putchar('o');
    putchar('s');
    unsafe {
        /*
        *UART_THR = 'v' as u8;
        asm!("strb $1, [$0]"
        : // ouputs
        : "r"(UART_THR), "r"('d' as u8) // inputs
        : // clobbers
        : // options
        );
        while (read_volatile(UART_LSR) & (1 << 5)) == 0 {}
        write_volatile(UART_THR, 'v' as u8);
        */
    }
}

fn putchar(ch: char) {
    //let a = ch as u8 + 1;
    unsafe {
        while (read_volatile(UART_LSR) & (1 << 5)) == 0 {}
        write_volatile(UART_THR, ch as u8);
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {loop{}}

// something is inserting annoying functions in my code
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

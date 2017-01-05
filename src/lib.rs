#![feature(asm)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate rlibc;
extern crate compiler_builtins;

use core::fmt::Write;

mod uart;
mod gpio;

#[no_mangle]
pub extern "C" fn rust_main() {
    write!(uart::UART0, "hello {}\r\n", "Pepijn").unwrap();
    unsafe {
        gpio::init_gpio();
    }
    write!(uart::UART0, "GPIO init done\r\n").unwrap();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

// something is inserting annoying functions in my code
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}

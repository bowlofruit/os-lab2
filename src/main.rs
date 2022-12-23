#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buf;

use core::fmt::Write;
use core::panic::PanicInfo;
use crate::vga_buf::{Alignment, Color, Screen};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> !
{

    let mut screen = Screen::new(Color::Yellow as u8,Color::Blue as u8, Alignment::Center);
    for i in 0..100 {
        write!(screen, "Number {}\n", i);
    }

    loop {}
}
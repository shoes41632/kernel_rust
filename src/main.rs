#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod screen;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    screen::clear_screen(vga_buffer);
    loop {}
}

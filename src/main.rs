#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga_buffer::WRITER
        .lock()
        .change_colors(vga_buffer::Color::White, vga_buffer::Color::Red);
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");
    loop {}
}

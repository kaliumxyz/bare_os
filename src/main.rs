#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

// function to be called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world{}", "!");

    loop {}
}

#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

// function to be called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("hello!").unwrap();
    write!(vga_buffer::WRITER.lock(), ", numbers are {} and {}", 42, 1.0/3.0).unwrap();

    loop {}
}

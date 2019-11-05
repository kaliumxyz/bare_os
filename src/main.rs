#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(octarine::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use octarine::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world{}", "!");

    octarine::init();


    // invoke breakpoint exception
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Welcome!");
    loop {}
}

// function to be called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    octarine::test_panic_handler(info)
}


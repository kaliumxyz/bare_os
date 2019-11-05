#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use octarine::{serial_print, serial_println, exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {

    serial_print!("stack_overflow... ");

    octarine::gdt::init();
    init_test_idt();

    stack_overflow();
    panic!("execution continued after stack overflow");
}

use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(octarine::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// overflow the stack to cause a triple fault
#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    octarine::test_panic_handler(info)
}


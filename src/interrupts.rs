use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_name() {
    serial_print!("test_breakpoint_exception...");
    // invoke breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}

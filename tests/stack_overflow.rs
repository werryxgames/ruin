#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use ruin::{exit_qemu, QemuExitCode, serial_println};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ruin::panic_test(info);
}

lazy_static! {
    static ref STATIC_TEST_IDT: InterruptDescriptorTable = {
        let mut test_idt = InterruptDescriptorTable::new();
        unsafe { test_idt.double_fault.set_handler_fn(test_double_fault_handler).set_stack_index(ruin::gdt::IST_INDEX); }
        test_idt
    };
}

pub fn init_test_idt() {
    STATIC_TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("Pass");
    exit_qemu(QemuExitCode::Ok);
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    ruin::gdt::init();
    init_test_idt();
    stack_overflow();

    panic!("Fail");
}

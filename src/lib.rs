#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga;
pub mod interrupts;
pub mod gdt;
pub mod keyboard;
pub mod pci;
#[path = "./storage/ata_pio.rs"]
pub mod ata_pio;
#[path = "./acpi/acpi.rs"]
pub mod acpi;

use core::panic::PanicInfo;

use x86_64::instructions::port::Port;

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn panic_test(info: &PanicInfo) -> ! {
    serial_println!("Fail: {}", info);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Fail);
    loop {}
}

pub trait Testable {
    fn test(&self, test_id: usize) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn test(&self, test_id: usize) {
        serial_print!("Test #{}: {}: ", test_id, core::any::type_name::<T>());
        self();
        serial_println!("Pass");
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Total tests: {}", tests.len());
    let mut current_test: usize = 0;

    for test in tests {
        current_test += 1;
        test.test(current_test);
    }

    serial_println!("Passed all {} tests", tests.len());
    exit_qemu(QemuExitCode::Ok);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Ok = 0x10,
    Fail = 0x11
}

pub fn exit_qemu(code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xF4);
        port.write(code as u32);
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS_MUTEX.lock().initialize(); }
    x86_64::instructions::interrupts::enable();
    acpi::find_xsdp_bios();
    // ata_pio::initialize();
    println!("Vendor: {}", pci::check_vendor(0, 0));
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    halt_loop();
}

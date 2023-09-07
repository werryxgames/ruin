#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec::Vec};
use ruin::{halt_loop, serial_println, println, memory, allocator};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    println!("{}", info);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ruin::panic_test(info)
}

pub fn kernel_start(boot_info: &'static BootInfo) -> ! {
    ruin::init();

    serial_println!("RuinOS by Werryx Games");
    println!("RuinOS by Werryx Games");

    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { memory::MemoryMapFrameAllocator::new(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
    let x = Box::new(42);
    println!("Box ptr = {:p}", x);
    let mut vec = Vec::new();

    for i in 0..500 {
        vec.push(i);
    }

    println!("Vec ptr = {:p}", vec.as_slice());
    allocator::map_physical(&mut mapper, 0xE0000, 0xFFFFF).unwrap();

    if ruin::acpi::find_xsdp_bios().is_some() {
        println!("Found XSDP")
    } else {
        println!("Not found XSDP");
    };

    // ata_pio::initialize();
    println!("Vendor: {}", ruin::pci::check_vendor(0, 0));

    #[cfg(test)]
    test_main();

    println!("Entered infinite loop");
    halt_loop();
}

entry_point!(kernel_start);


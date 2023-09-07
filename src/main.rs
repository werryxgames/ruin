#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use core::panic::PanicInfo;
use ruin::{serial_println, println, memory, allocator, task::{executor::Executor, Task}};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use ruin::halt_loop;

    serial_println!("{}", info);
    println!("{}", info);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ruin::panic_test(info)
}

async fn async_number() -> u8 {
    42
}

async fn async_print_number() {
    println!("Async: {}", async_number().await);
}

pub fn kernel_start(boot_info: &'static BootInfo) -> ! {
    ruin::init();

    serial_println!("RuinOS by Werryx Games");
    println!("RuinOS by Werryx Games");

    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { memory::MemoryMapFrameAllocator::new(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    #[cfg(test)]
    test_main();

    allocator::map_physical(&mut mapper, 0xE0000, 0x1FFFF).unwrap();

    if ruin::acpi::find_xsdp_bios().is_some() {
        println!("Found XSDP")
    } else {
        println!("Not found XSDP");
    };

    // ata_pio::initialize();
    println!("Vendor: {}", ruin::pci::check_vendor(0, 0));

    let mut executor = Executor::new();
    executor.spawn(Task::new(async_print_number()));
    executor.spawn(Task::new(ruin::task::keyboard::print_keypress()));
    executor.run();
}

entry_point!(kernel_start);


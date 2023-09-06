#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ruin::{halt_loop, serial_println, println, memory};
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Page, VirtAddr};

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
    let page = Page::containing_address(VirtAddr::new(0));
    memory::map_page(page, &mut mapper, &mut frame_allocator);
    unsafe { (page.start_address().as_mut_ptr() as *mut u64).offset(400).write_volatile(0x_f021_f077_f065_f04e) }

    #[cfg(test)]
    test_main();

    println!("Entered infinite loop");
    halt_loop();
}

entry_point!(kernel_start);


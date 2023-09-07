#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use ruin::{memory::{self, MemoryMapFrameAllocator}, allocator::{self, HEAP_SIZE}};
use x86_64::VirtAddr;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ruin::panic_test(info);
}

fn main(boot_info: &'static BootInfo) -> ! {
    ruin::init();
    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { MemoryMapFrameAllocator::new(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    test_main();

    loop {}
}

entry_point!(main);

#[test_case]
fn test_check_alloc_values() {
    let heap_val1 = Box::new(42);
    let heap_val2 = Box::new(7);
    assert_eq!(*heap_val1, 42);
    assert_eq!(*heap_val2, 7);
}

#[test_case]
fn test_vec() {
    const ITERS: u64 = 1024;
    let mut vec = Vec::new();

    for i in 0..ITERS {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), ((ITERS - 1) * ITERS / 2));
}

#[test_case]
fn test_full_memory() {
    for i in 0..HEAP_SIZE {
        let box_ = Box::new(i);
        assert_eq!(*box_, i);
    }
}

#[test_case]
fn test_full_memory_long_lived() {
    let long_lived = Box::new(321);

    for i in 0..HEAP_SIZE {
        let box_ = Box::new(i);
        assert_eq!(*box_, i);
    }

    assert_eq!(*long_lived, 321);
}

#[test_case]
fn test_fragmentation_fast() {
    let mut layouts: Vec<(*mut u8, alloc::alloc::Layout, u8)> = Vec::new();

    for i in 0..120 {
        let layout = alloc::alloc::Layout::from_size_align(HEAP_SIZE / 128, HEAP_SIZE / 128).unwrap();
        let ptr: *mut u8;

        unsafe {
            ptr = alloc::alloc::alloc(layout);
            ptr.write(i as u8);
        }

        layouts.push((ptr, layout, i as u8));
    }

    for layout in layouts {
        unsafe { assert_eq!(layout.0.read(), layout.2); }
        unsafe { alloc::alloc::dealloc(layout.0, layout.1); }
    }

    let layout = alloc::alloc::Layout::from_size_align(HEAP_SIZE * 18 / 20, 1).unwrap();

    unsafe {
        let ptr = alloc::alloc::alloc(layout);
        ptr.write(ptr as u8);
        assert_eq!(ptr.read(), ptr as u8);
        alloc::alloc::dealloc(ptr, layout);
    }
}

#[test_case]
fn test_fragmentation_full() {
    let mut layouts: Option<Vec<*mut Box<u8>>> = Some(Vec::new());

    for i in 0..HEAP_SIZE / 16 { // / 2 (half) / 8 (64 bit address) = full memory
        let mut box_ = Box::new(i as u8);
        layouts.as_mut().unwrap().push(&mut box_);
        assert_eq!(*box_, i as u8);
    }

    layouts = None;
    assert_eq!(layouts, None);

    let layout = alloc::alloc::Layout::from_size_align(HEAP_SIZE / 2 + 2, 2).unwrap(); // + 2 to make test fail if fragmentation is incorrect even in 32 bit OS

    unsafe {
        let ptr = alloc::alloc::alloc(layout);
        ptr.write(ptr as u8);
        assert_eq!(ptr.read(), ptr as u8);
        alloc::alloc::dealloc(ptr, layout);
    }
}

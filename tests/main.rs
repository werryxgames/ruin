#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::fmt::Write;
use core::ptr::NonNull;
use volatile::VolatileRef;
use x86_64::instructions::interrupts;
use ruin::vga::WRITER;
use ruin::vga::BUFFER_WIDTH;
use ruin::vga::BUFFER_HEIGHT;
use ruin::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ruin::panic_test(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println_once() {
    println!("Hello, World!");
    panic!("Should fail");
}

#[test_case]
fn test_println_overflow() {
    for i in 0..BUFFER_WIDTH + 1 {
        println!("Line {}", i);
    }
}

#[test_case]
fn test_println_text() {
    let string = "Test string";

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", string).unwrap();

        for (i, char) in string.chars().enumerate() {
            let real_char;
            unsafe { real_char = VolatileRef::new(NonNull::new(&mut writer.buffer.chars[BUFFER_HEIGHT - 2][i]).unwrap()).as_ptr().read() };
            assert_eq!(char::from(real_char.character), char);
        }
    });
}

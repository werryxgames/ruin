#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ruin::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ruin::{halt_loop, serial_println};
use ruin::println;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    ruin::init();

    serial_println!("RuinOS by Werryx Games");
    println!("RuinOS by Werryx Games");

    #[cfg(test)]
    test_main();

    println!("Entered infinite loop");
    halt_loop();
}

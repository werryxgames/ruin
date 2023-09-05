#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ruin::serial_println;
use ruin::exit_qemu;
use ruin::QemuExitCode;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("Pass");
    exit_qemu(QemuExitCode::Ok);
    loop {}
}

// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Total tests: {}", tests.len());
//
//     for (i, test) in tests.iter().enumerate() {
//         serial_print!("Test #{}: ", i + 1);
//         test();
//         serial_println!("Fail");
//         exit_qemu(QemuExitCode::Fail);
//     }
//
//     exit_qemu(QemuExitCode::Ok);
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_should_panic();
    serial_println!("Fail");
    exit_qemu(QemuExitCode::Fail);
    loop {}
}

fn test_should_panic() {
    assert_eq!(1 + 1, 1 * 1);
}

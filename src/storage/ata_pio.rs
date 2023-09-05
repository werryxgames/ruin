use x86_64::instructions::port::Port;
use crate::println;

pub fn initialize() -> bool {
    let mut _data_reg_port: Port<u16> = Port::new(0x1F0);
    let mut _error_reg_port: Port<u16> = Port::new(0x1F1);
    let mut _features_reg_port: Port<u16> = Port::new(0x1F1);
    let mut sectorcount_port: Port<u16> = Port::new(0x1F2);
    let mut lbalo_port: Port<u16> = Port::new(0x1F3);
    let mut lbamid_port: Port<u16> = Port::new(0x1F4);
    let mut lbahi_port: Port<u16> = Port::new(0x1F5);
    let mut drive_select_port: Port<u8> = Port::new(0x1F6);
    let mut status_port: Port<u8> = Port::new(0x1F7);
    let mut command_reg_port: Port<u8> = Port::new(0x1F7);
    let mut status: u8;

    unsafe {
        sectorcount_port.write(0x12);
        lbalo_port.write(0x34);
        println!("{}, {}", sectorcount_port.read(), lbalo_port.read());

        if sectorcount_port.read() == 0x12 || lbalo_port.read() != 0x34 {
            println!("No ATA PIO controller");
            return false;
        }

        drive_select_port.write(0xA0);
        sectorcount_port.write(0x00);
        lbalo_port.write(0x00);
        lbamid_port.write(0x00);
        lbahi_port.write(0x00);
        command_reg_port.write(0xEC);
        status = status_port.read();
    }

    if status == 0x00 {
        println!("No ATA PIO compatible hard drives detected");
        return false;
    }

    while status | 64 != status {
        println!("poll");
        unsafe {
            if lbamid_port.read() != 0x00 || lbahi_port.read() != 0x00 {
                println!("Not ATA");
                return false;
            }

            status = status_port.read();
        }
    }

    println!("ATA PIO found");
    return true;
}


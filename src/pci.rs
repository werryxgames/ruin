use x86_64::instructions::port::Port;

pub fn read_config_u16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let lbus: u32 = bus as u32;
    let lslot: u32 = slot as u32;
    let lfunc: u32 = func as u32;

    let address1: u32 = (lbus << 16) | (lslot << 11);
    let address: u32;

    if address1 > 0 {
        address = address1;
    } else {
        address = (lfunc << 8) | (offset & 0xFC) as u32 | (0x80000000 as u32);
    }

    unsafe { Port::<u32>::new(0xCF8).write(address); }
    let result: u32;
    unsafe { result = Port::<u32>::new(0xCFC).read(); }
    return ((result >> ((offset & 2) << 3)) & 0xFFFF) as u16; // << 3 == * 8
}

pub fn check_vendor(bus: u8, slot: u8) -> u16 {
    let vendor: u16 = read_config_u16(bus, slot, 0, 0);
    let device: u16;

    if vendor != 0xFFFF {
        device = read_config_u16(bus, slot, 0, 2);
    }

    return vendor;
}

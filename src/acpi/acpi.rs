#[repr(C, packed)]
pub struct xsdp {
    pub signature: [char; 8],
    pub checksum: u8,
    pub oemiud: [char; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub checksum2: u8,
    pub _reserved: [u8; 3]
}

pub unsafe fn check_xsdp(xsdp_ptr: *const xsdp) -> bool {
    let mut sum: u8 = 0;

    for i in 0..20 {
        unsafe { sum += *(xsdp_ptr.add(i) as *const u8); }
    }

    if sum != 0 {
        return false;
    }

    if (*xsdp_ptr).revision < 2 { // 0 = ACPI 1.0, 1 = unknown, 2+ = ACPI 2.0+
        // is RSDP
        return true;
    }

    let mut sum2: u8 = 0;

    for i in 0..36 {
        unsafe { sum2 += *(xsdp_ptr.add(i) as *const u8); }
    }

    return sum2 == 0;
}

pub fn find_xsdp_bios() -> Option<*const xsdp> {
    for mem in 0xE0000..0xFFFFF - 20 { // 20 is the size of ACPI 1.0 XSDP (RSDP) table
        unsafe {
            let string: *const char = mem as *const char;

            if *string == 'R' && *string.add(1) == 'S' && *string.add(2) == 'D' && *string.add(3) == ' ' && *string.add(4) == 'P' && *string.add(5) == 'T' && *string.add(6) == 'R' && *string.add(7) == ' ' {
                if check_xsdp(string as *const xsdp) {
                    return Some(string as *const xsdp);
                }
            }
        }
    }

    None
}

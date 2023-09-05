use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;

use lazy_static::lazy_static;
use x86_64::instructions::tables::load_tss;
use x86_64::instructions::segmentation::{CS, DS, Segment};
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::SegmentSelector;

pub const IST_INDEX: u16 = 0; // [0; 7]

lazy_static! {
    static ref STATIC_TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK }); // TODO: Make stack allocation
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

struct GdtSelectors {
    kernel_code_selector: SegmentSelector,
    kernel_data_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

lazy_static! {
    static ref STATIC_GDT: (GlobalDescriptorTable, GdtSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&STATIC_TSS));
        (gdt, GdtSelectors { kernel_code_selector, kernel_data_selector, tss_selector })
    };
}

pub fn init() {
    STATIC_GDT.0.load();

    unsafe {
        CS::set_reg(STATIC_GDT.1.kernel_code_selector);
        DS::set_reg(STATIC_GDT.1.kernel_data_selector);
        load_tss(STATIC_GDT.1.tss_selector);
    }
}

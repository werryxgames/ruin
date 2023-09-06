use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr,
    VirtAddr,
    registers::control::Cr3,
    structures::paging::{Page, Mapper, Size4KiB, PhysFrame, FrameAllocator, OffsetPageTable, PageTable, page_table::FrameError, PageTableFlags}
};

unsafe fn get_current_page_table(physical_offset: VirtAddr) -> &'static mut PageTable {
    let (current_table, _) = Cr3::read();
    let physical_mem = current_table.start_address();
    let virtual_mem = physical_offset + physical_mem.as_u64();
    let page_table_ptr: *mut PageTable = virtual_mem.as_mut_ptr();

    &mut *page_table_ptr
}

fn _get_physical_address(virtual_address: VirtAddr, physical_offset: VirtAddr) -> Option<PhysAddr> {
    let (current_table, _) = Cr3::read();
    let indexes = [virtual_address.p4_index(), virtual_address.p3_index(), virtual_address.p2_index(), virtual_address.p1_index()];
    let mut table_frame = current_table;

    for &index in &indexes {
        let virtual_mem = physical_offset + table_frame.start_address().as_u64();
        let table_ptr: *const PageTable = virtual_mem.as_ptr();
        let table = unsafe {&*table_ptr};
        let entry = &table[index];
        table_frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge pages are not supported")
        };
    }

    Some(table_frame.start_address() + u64::from(virtual_address.page_offset()))
}

pub unsafe fn get_physical_address(virtual_address: VirtAddr, physical_offset: VirtAddr) -> Option<PhysAddr> {
    _get_physical_address(virtual_address, physical_offset)
}

pub fn map_page(page: Page, mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xB8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {mapper.map_to(page, frame, flags, frame_allocator)}.unwrap().flush();
}

pub unsafe fn init(physical_offset: VirtAddr) -> OffsetPageTable<'static> {
    let l4_table = get_current_page_table(physical_offset);
    OffsetPageTable::new(l4_table, physical_offset)
}

pub struct MemoryMapFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize
}

impl MemoryMapFrameAllocator {
    fn get_usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|reg| reg.region_type == MemoryRegionType::Usable);
        let usable_addresses = usable_regions.map(|reg| reg.range.start_addr()..reg.range.end_addr());
        let frame_addresses = usable_addresses.flat_map(|addr| addr.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        MemoryMapFrameAllocator { memory_map, next: 0 }
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemoryMapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.get_usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}


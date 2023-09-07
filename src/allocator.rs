pub mod linked_list;

use spin::{Mutex, MutexGuard};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, PhysFrame
    },
    VirtAddr, PhysAddr
};
use linked_list_allocator::LockedHeap;

use crate::memory::EmptyFrameAllocator;

pub const HEAP_START: usize = 0x44444444_0000;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn map_range(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>, start: usize, size: usize) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(start as u64);
        let heap_end = heap_start + size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe { ALLOCATOR.lock().init(start as *mut u8, size); }

    Ok(())
}

pub fn map_physical(mapper: &mut impl Mapper<Size4KiB>, start: usize, size: usize) -> Result<(), MapToError<Size4KiB>> {
    let frame_range = {
        let phys_start = PhysAddr::new(start as u64);
        let phys_end = phys_start + size - 1u64;
        let phys_start_page = PhysFrame::containing_address(phys_start);
        let phys_end_page = PhysFrame::containing_address(phys_end);
        PhysFrame::range_inclusive(phys_start_page, phys_end_page)
    };

    let mut frame_allocator = EmptyFrameAllocator::new();

    for frame in frame_range {
        let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64()));
        let flags: PageTableFlags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator) }?.flush();
    }
    
    Ok(())
}

pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    map_range(mapper, frame_allocator, HEAP_START, HEAP_SIZE)
}

pub struct Locked<A> {
    inner: Mutex<A>
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked { inner: Mutex::new(inner) }
    }

    pub fn lock(&self) -> MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up_power2(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

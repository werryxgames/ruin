use core::{mem, alloc::{GlobalAlloc, Layout}, ptr};

use crate::allocator::align_up_power2;

use super::Locked;

struct Node {
    next: Option<&'static mut Node>,
    size: usize
}

impl Node {
    const fn new(size: usize) -> Self {
        Node { next: None, size }
    }

    fn get_start_address(&self) -> usize {
        self as *const Self as usize
    }

    fn get_end_address(&self) -> usize {
        self.get_start_address() + self.size
    }
}

pub struct LinkedListAllocator {
    head: Node
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: Node::new(0)
        }
    }

    unsafe fn free_region(&mut self, start: usize, size: usize) {
        assert_eq!(align_up_power2(start, mem::align_of::<Node>()), start);
        assert!(size >= mem::size_of::<Node>());

        let mut node = Node::new(size);
        node.next = self.head.next.take();
        let node_ptr = start as *mut Node;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr)
    }

    fn alloc_region(region: &Node, size: usize, align: usize) -> Result<usize, ()> {
        let start = align_up_power2(region.get_start_address(), align);
        let end = start.checked_add(size).ok_or(())?;

        if end > region.get_end_address() {
            return Err(());
        }

        let excess_size = region.get_end_address() - end;

        if excess_size > 0 && excess_size < mem::size_of::<Node>() {
            return Err(());
        }

        Ok(start)
    }

    fn next_free_region(&mut self, size: usize, align: usize) -> Option<(&'static mut Node, usize)> {
        let mut current_node = &mut self.head;

        while let Some(ref mut region) = current_node.next {
            if let Ok(address) = Self::alloc_region(&region, size, align) {
                let next = region.next.take();
                let result = Some((current_node.next.take().unwrap(), address));
                current_node.next = next;
                return result;
            } else {
                current_node = current_node.next.as_mut().unwrap();
            }
        }

        None
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout.align_to(mem::align_of::<Node>()).unwrap().pad_to_align();
        let size = layout.size().max(mem::size_of::<Node>());
        (size, layout.align())
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.free_region(heap_start, heap_size);
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, start)) = allocator.next_free_region(size, align) {
            let end = start.checked_add(size).unwrap();
            let excess_size = region.get_end_address() - end;

            if excess_size > 0 {
                allocator.free_region(end, excess_size);
            }

            start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);
        self.lock().free_region(ptr as usize, size)
    }
}

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use crate::{gdt, println, keyboard};
use spin::Mutex;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 32 + 8;
pub static PICS_MUTEX: Mutex<ChainedPics> = Mutex::new(unsafe {ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET)});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HardwareInterrupt {
    Timer = PIC1_OFFSET,
    Keyboard
}

impl HardwareInterrupt {
    fn to_u8(self) -> u8 {
        self as u8
    }

    fn to_usize(self) -> usize {
        usize::from(self.to_u8())
    }
}

lazy_static! {
    static ref STATIC_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(on_breakpoint);
        idt.page_fault.set_handler_fn(on_page_fault);
        unsafe { idt.double_fault.set_handler_fn(on_double_fault).set_stack_index(gdt::IST_INDEX); }
        idt[HardwareInterrupt::Timer.to_usize()].set_handler_fn(on_hardware_timer);
        idt[HardwareInterrupt::Keyboard.to_usize()].set_handler_fn(on_hardware_keyboard);

        idt
    };
}

extern "x86-interrupt" fn on_breakpoint(stack_frame: InterruptStackFrame) {
    println!("Breakpoint: {:#?}", stack_frame);
}

extern "x86-interrupt" fn on_page_fault(stack_frame: InterruptStackFrame, code: PageFaultErrorCode) {
    println!("Page fault {:?}: {:#?}\nAccessed address: {:?}", code, stack_frame, Cr2::read());
}

extern "x86-interrupt" fn on_double_fault(stack_frame: InterruptStackFrame, code: u64) -> ! {
    panic!("Double fault ({}): {:#?}", code, stack_frame);
}

extern "x86-interrupt" fn on_hardware_timer(_stack_frame: InterruptStackFrame) {
    unsafe { PICS_MUTEX.lock().notify_end_of_interrupt(HardwareInterrupt::Timer.to_u8()); }
}

extern "x86-interrupt" fn on_hardware_keyboard(_stack_frame: InterruptStackFrame) {
    let mut port: Port<u8> = Port::new(0x60);
    keyboard::handle_key_press(&mut port);
    unsafe { PICS_MUTEX.lock().notify_end_of_interrupt(HardwareInterrupt::Keyboard.to_u8()); }
}

pub fn init_idt() {
    STATIC_IDT.load();
}

#[test_case]
fn test_breakpoint() {
    x86_64::instructions::interrupts::int3();
}

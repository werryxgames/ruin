use core::fmt;
use core::fmt::Write;
use core::ptr::NonNull;

use spin::Mutex;
use volatile::VolatileRef;

use lazy_static::lazy_static;
use x86_64::instructions::interrupts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub fn new(foreground: VgaColor, background: VgaColor) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaChar {
    pub character: u8,
    color: ColorCode
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct VgaBuffer {
    pub chars: [[VgaChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct VgaWriter {
    column: usize,
    color: ColorCode,
    pub buffer: &'static mut VgaBuffer
}

impl VgaWriter {
    pub fn new(foreground: VgaColor, background: VgaColor) -> VgaWriter {
        VgaWriter { column: 0, color: ColorCode::new(foreground, background), buffer: unsafe { &mut *(0xB8000 as *mut VgaBuffer) } }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column;
                let color = self.color;

                unsafe {
                    VolatileRef::new(NonNull::new(&mut self.buffer.chars[row][col]).unwrap()).as_mut_ptr().write(VgaChar {
                        character: byte,
                        color
                    })
                };
                self.column += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xFE)
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let empty = VgaChar {
            character: b' ',
            color: self.color
        };

        for col in 0..BUFFER_WIDTH {
            unsafe { VolatileRef::new(NonNull::new(&mut self.buffer.chars[row][col]).unwrap()).as_mut_ptr().write(empty); }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col]; // non-volatile read because of volatile write line below
                unsafe { VolatileRef::new(NonNull::new(&mut self.buffer.chars[row - 1][col]).unwrap()).as_mut_ptr().write(character); }
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new(VgaColor::LightGray, VgaColor::Black));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| { // To prevent deadlock on interrupt
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

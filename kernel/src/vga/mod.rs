//!
//! file
//! : vga/mod.rs
//!
//! desc
//! : Main file for the VGA_BUFFER module. Contains structures and functions
//! for writing to the screen.
//!
//! note
//! : Taken from blog_os
//!
//! changelog
//!
//! - 2017-08-04: file created
//!
#![macro_use]

// use definitions
use core::fmt;
use core::ptr::Unique;
use volatile::Volatile;
use spin::Mutex;

// Constants
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH:  usize = 80;

// Other code
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum VGAColor{
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct VGAColorCode(u8);

impl VGAColorCode {
    const fn new(foreground: VGAColor, background: VGAColor) -> VGAColorCode {
        VGAColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VGAEntry {
    ascii_char: u8,
    color_code: VGAColorCode,
}

pub struct VGABuffer {
    chars: [Volatile<[VGAEntry; BUFFER_WIDTH]>; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    column_position: usize,
    color_code: VGAColorCode,
    buffer: Unique<VGABuffer>,
}

impl ScreenWriter {
    /// write a single byte to the current position of the writer
    pub fn putchar(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                // READ
                let mut row_chars = self.buffer().chars[row].read();

                // MODIFY
                row_chars[col] = VGAEntry {
                    ascii_char: byte,
                    color_code: color_code,
                };

                // WRITE
                self.buffer().chars[row].write(row_chars);

                self.column_position += 1;
            }
        }
    }

    /// write a full string starting at the current position of the writer
    pub fn putstr(&mut self, s: &str) {
        for (idx, line) in s.split("\n").enumerate() {
            if idx != 0 {
                self.new_line();
            }

            if line.len() == 0 {
                continue;
            }

            let row = BUFFER_HEIGHT - 1;
            let color_code = self.color_code;

            // READ
            let mut row_chars = self.buffer().chars[row].read();

            // MODIFY
            for byte in line.bytes() {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let col = self.column_position;
                row_chars[col] = VGAEntry {
                    ascii_char: byte,
                    color_code: color_code,
                };
                self.column_position += 1;
            }

            // WRITE
            self.buffer().chars[row].write(row_chars);
        }
    }

    /// get a mutable reference to the buffer
    fn buffer(&mut self) -> &mut VGABuffer {
        unsafe{ self.buffer.as_mut() }
    }

    /// move everything up one row and clear the bottom row
    fn new_line(&mut self) {
        // moves chars[..1] to the end and chars[1..] to the beginning (with a memcpy)
        self.buffer().chars.rotate(1);
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    /// clear a single row on the screen by overwriting with ' ' 
    fn clear_row(&mut self, row: usize) {
        let blank = VGAEntry {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        let blank_row = [blank; BUFFER_WIDTH];
        self.buffer().chars[row].write(blank_row);
    }
}

/// Let us use the write! macro
impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putchar(byte);
        }
        Ok(())
    }
}

/// Static writer interface
pub static WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
    column_position: 0,
    color_code: VGAColorCode::new(VGAColor::LightGreen, VGAColor::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

/// print macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::print(format_args!($($arg)*));
    });
}

/// println macro
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Helper function for printing
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Function to clear the screen
pub fn clear_screen() {
    for row in 0..BUFFER_HEIGHT {
        WRITER.lock().clear_row(row);
    }
}

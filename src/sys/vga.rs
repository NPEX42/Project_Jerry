
// VGA Textmode 80x25.
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

// Base VGA Text Buffer Address.
pub const VGA_BUFFER: *mut Buffer = 0xb8000 as *mut Buffer;

const CHAR_SPACE: u8 = b' ';
const CHAR_TAB: u8 = b'\t';
const CHAR_DEL: u8 = b'\x1b';
const CHAR_UNK: u8 = b'\xfe';

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Packs two [Color] values into a single vga color attribute.
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode(((bg as u8) << 4) | fg as u8) // Bbbbffff
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

#[repr(C)]
pub struct ScreenChar {
    ascii_code: u8,
    color: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct TerminalWriter {
    column_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, 
}

impl TerminalWriter {
    pub fn new() -> TerminalWriter {
        TerminalWriter {
            column_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Blue),
            buffer: unsafe { &mut *VGA_BUFFER},
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_code: byte,
                    color: color_code,
                };
                self.column_pos += 1;
            }
        }
    }
    pub fn write_byte_at(&mut self, byte: u8, x: usize, y: usize) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                let color_code = self.color_code;
                self.buffer.chars[y][x] = ScreenChar {
                    ascii_code: byte,
                    color: color_code,
                };
            }
        }
    }

    fn new_line(&mut self) {/* TODO */}

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(CHAR_UNK),
            }

        }
    }

    pub fn clear_line(&mut self, y: usize) {
        for x in 0..BUFFER_WIDTH {
            self.write_byte_at(CHAR_SPACE, x, y);
        }
    }

    pub fn clear_screen(&mut self) {
        for y in 0..BUFFER_HEIGHT {
            self.clear_line(y);
        }
        self.column_pos = 0;
    }
}
// Enum to specify the number for each colour.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
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

// Represents a full colour code.
// It contains the foreground and background colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);


impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Represents a character on the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode
}

// Represents the text buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Lets us actually write to the screen.
pub struct Writer {
    column_position: usize,
    color_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn new_line() {/* TODO */}

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range.
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => Writer::new_line(),
            byte => {
                // Check if the line is full. If it is, make a new line before writing.
                if self.column_position >= BUFFER_WIDTH {
                    Writer::new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.color_code;

                // Write a ScreenChar to the buffer at the current position.
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    colour_code,
                };

                // Advance to the next column position.
                self.column_position += 1;
            }
        }
    }
}

pub fn write_test() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // The VGA Buffer.
    };

    writer.write_string("Hello ");
    writer.write_string("YeffOS!");
}
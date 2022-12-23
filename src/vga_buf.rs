const BUF_ADDR: u32 = 0xb8000;
const BUF_HEIGHT: u32 = 25;
const BUF_WIDTH: u32 = 80;

const COLOR_BLACK: u8 = 0x0;

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8
}

pub enum Alignment {
    Left,
    Right,
    Center
}

pub struct Screen {
    buffer: *mut u8,
    color: u8,
    background_color: u8,
    align: Alignment,
    cursor_row: u32,
    cursor_col: u32,
    state : [[u8; BUF_WIDTH as usize]; BUF_HEIGHT as usize],
    cursor_write: u32
}
impl core::fmt::Write for Screen
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        self.print(s);
        Ok(())
    }
}
impl Screen
{
    pub fn new(color: u8, background_color: u8, align: Alignment) -> Screen
    {
        return Screen
        {
            background_color: (COLOR_BLACK << 4) | background_color,
            buffer: BUF_ADDR as *mut u8,
            color: (COLOR_BLACK << 4) | color,
            align,
            cursor_row: 0,
            cursor_col: 0,
            state : [[0; BUF_WIDTH as usize]; BUF_HEIGHT as usize],
            cursor_write: 0
        }
    }
    pub fn write_char(&mut self, offset: u32, char: AsciiChar)
    {
        unsafe
            {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
        }

        self.cursor_write += 1;
    }

    pub fn read_char(&self, offset: u32) -> AsciiChar
    {
        unsafe
            {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }

    pub fn print(&mut self, s: &str)
    {
        self.add_text(s.as_bytes());
        self.cursor_write = 0;
        for row in self.state
        {
            let align = self.calc_align(&row);
            for i in 0..align
            {
                self.write_char(
                    self.cursor_write, AsciiChar{char_byte : b' ', color_byte: self.color}
                );
            }
            for c in row {
                if c == b'\0' {
                    break;
                }
                self.write_char(
                    self.cursor_write, AsciiChar{char_byte : c, color_byte: self.color}
                );
            }
            self.cursor_write += BUF_WIDTH - (self.cursor_write % BUF_WIDTH);
        }
    }

    pub fn add_text(&mut self, row: &[u8]) {
        for i in 0..row.len() {
            if self.cursor_row == BUF_WIDTH - 1 || row[i] == b'\n' {
                self.state[self.cursor_col as usize ][self.cursor_row as usize] = b'\0';
                self.cursor_col += 1;
                self.cursor_row = 0;

                if self.cursor_col == BUF_HEIGHT - 1 {
                    self.scroll();
                }

                continue;
            }
            self.state[self.cursor_col as usize][self.cursor_row as usize] = row[i];
            self.cursor_row += 1;
        }
    }

    pub fn scroll(&mut self) {
        for i in 0..self.cursor_col - 1
        {
            self.state[i as usize] = self.state[(i + 1) as usize];
        }

        for i in 0..self.state[self.cursor_col as usize].len()
        {
            self.state[(self.cursor_col) as usize][i] = b' ';
        }
        self.cursor_col -= 1;
    }
    pub fn calc_align(&self, row: &[u8]) -> u32
    {
        let mut len = 0;
        for c in row
        {
            if *c == b'\0'
            {
                break;
            }
            len += 1;
        }

        match self.align
        {
            Alignment::Left => 0,
            Alignment::Right => BUF_WIDTH - len,
            Alignment::Center => (BUF_WIDTH - len) / 2
        }
    }
}

#[repr(u8)]
pub enum Color {
    Blue = 0x1,
    Green = 0x2,
    Azure = 0x3,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    Light_grey = 0x7,
    Dark_grey = 0x8,
    Light_blue = 0x9,
    Light_green = 0xa,
    Light_azure = 0xb,
    Light_red = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf
}
use js_sys::Uint8ClampedArray;
use rand::Rng;
use wasm_bindgen::prelude::*;

mod color;

use crate::color::{Color, PALETTE};

pub type Row = Vec<Color>;
pub type Screen = Vec<Row>;

pub type BufferRow = Vec<u8>;
pub type Buffer = Vec<BufferRow>;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Fire {
    width: u16,
    height: u16,
    buffer: Buffer,
    is_running: bool,
}

#[wasm_bindgen]
impl Fire {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Fire {
        Fire {
            width,
            height,
            buffer: init_buffer(width, height),
            is_running: true,
        }
    }

    pub fn tick(&mut self) {
        tick(&mut self.buffer, self.width);
    }

    pub fn start(&mut self) {
        if self.is_running {
            return;
        };
        self.is_running = true;
        self.buffer[0] = vec![35; self.width as usize];
    }

    pub fn stop(&mut self) {
        if !self.is_running {
            return;
        };
        self.is_running = false;
        self.buffer[0] = vec![0; self.width as usize];
    }

    pub fn get_output(&self) -> Uint8ClampedArray {
        let output = &screen_to_byte_vec(&buffer_to_screen(&self.buffer))[..];
        Uint8ClampedArray::from(output)
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[wasm_bindgen(getter)]
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

fn init_buffer(width: u16, height: u16) -> Buffer {
    if width < 1 || height < 1 {
        panic!("Invalid dimensions!")
    }
    let first_row = vec![35; width as usize];
    let black_row = vec![0; width as usize];
    let mut buffer = vec![first_row];
    buffer.append(&mut vec![black_row; (height - 1) as usize]);
    buffer
}

fn buffer_row_to_screen_row(row: &BufferRow) -> Row {
    row.iter().map(|&i| PALETTE[i as usize]).collect()
}

fn buffer_to_screen(buffer: &Buffer) -> Screen {
    buffer.iter().map(buffer_row_to_screen_row).collect()
}

fn tick(buffer: &mut Buffer, width: u16) {
    for i in 1..buffer.len() {
        for j in 0..buffer[0].len() {
            let random: i8 = rand::thread_rng().gen();
            let wind: i32 = (random as i32 & 0b10) - 1;
            let prev = buffer[i - 1][j] as i16;
            let next = prev - (random as i16 & 1);
            let col = match j as i32 - wind {
                n if n < 0 => width - 1,
                n if n >= (width as i32) => 0,
                n => n.try_into().unwrap(),
            } as usize;
            buffer[i][col] = match next {
                n if n < 0 => 0,
                n if n as usize >= PALETTE.len() => PALETTE.len() - 1,
                _ => next as usize,
            } as u8;
        }
    }
}

fn row_to_byte_vec(row: &Row) -> Vec<u8> {
    row.iter().fold(vec![], |mut acc, c| {
        acc.append(&mut c.to_bytes().to_vec());
        acc
    })
}

fn screen_to_byte_vec(screen: &Screen) -> Vec<u8> {
    screen
        .iter()
        .rev()
        .map(row_to_byte_vec)
        .fold(vec![], |mut acc, mut r| {
            acc.append(&mut r);
            acc
        })
}

#[cfg(test)]
mod test {

    use super::{
        buffer_row_to_screen_row, buffer_to_screen, init_buffer, row_to_byte_vec,
        screen_to_byte_vec, tick, PALETTE,
    };

    #[test]
    fn init() {
        let [width, height] = [9, 10];
        let buffer = init_buffer(width, height);
        assert_eq!(buffer.len(), height as usize);
        buffer
            .iter()
            .for_each(|row| assert_eq!(row.len(), width as usize));
        let first_row = buffer.first().unwrap();
        first_row.iter().for_each(|&n| assert_eq!(n, 35));
        let rest = buffer.iter().skip(1);
        rest.for_each(|row| row.iter().for_each(|&n| assert_eq!(n, 0)));
    }

    #[test]
    #[should_panic(expected = "Invalid dimensions!")]
    fn invalid_height() {
        init_buffer(1, 0);
    }

    #[test]
    #[should_panic(expected = "Invalid dimensions!")]
    fn invalid_width() {
        init_buffer(0, 1);
    }

    #[test]
    fn buffer_row_conversion() {
        let row = vec![1, 2, 3, 4];
        let expected_colors = row.iter().map(|&i| PALETTE[i as usize]).collect::<Vec<_>>();
        let result = buffer_row_to_screen_row(&row);
        assert_eq!(row.len(), result.len());
        assert_eq!(result, expected_colors);
    }

    #[test]
    fn buffer_conversion() {
        let row = vec![1, 2, 3, 4];
        let expected_colors = row.iter().map(|&i| PALETTE[i as usize]).collect::<Vec<_>>();
        let buffer = vec![row; 5];
        let screen = buffer_to_screen(&buffer);
        assert_eq!(screen.len(), 5);
        screen.iter().for_each(|row| {
            assert_eq!(row.len(), 4);
            assert_eq!(*row, expected_colors);
        });
    }

    #[test]
    fn row_to_bytes() {
        let row = [1, 2, 3, 4].map(|i| PALETTE[i]).to_vec();
        let result = row_to_byte_vec(&row);
        assert_eq!(result.len(), 4 * 4);

        result.chunks(4).zip(row).for_each(|(bytes, color)| {
            assert_eq!(bytes[0], color.r);
            assert_eq!(bytes[1], color.g);
            assert_eq!(bytes[2], color.b);
            assert_eq!(bytes[3], color.a);
        });
    }

    #[test]
    fn screen_to_bytes() {
        let screen = (1..5)
            .map(|n| (n..n + 5).map(|i| PALETTE[i]).collect())
            .collect::<Vec<_>>();
        let result = screen_to_byte_vec(&screen);
        assert_eq!(result.len(), 4 * 5 * 4); // num of rows * row length * 4 bytes per pixel

        result
            .chunks(5 * 4)
            .zip(screen.iter().rev())
            .for_each(|(row, screen_row)| {
                assert_eq!(row.len(), screen_row.len() * 4);
                row.chunks(4).zip(screen_row).for_each(|(bytes, color)| {
                    assert_eq!(bytes[0], color.r);
                    assert_eq!(bytes[1], color.g);
                    assert_eq!(bytes[2], color.b);
                    assert_eq!(bytes[3], color.a);
                });
            });
    }

    #[test]
    fn tick_tock() {
        let [width, height] = [100, 6];
        let mut buffer = init_buffer(width, height);
        let mut previous = buffer.clone();
        for _ in 1..10 {
            tick(&mut buffer, width);
            assert_eq!(buffer.len(), height as usize);
            buffer
                .iter()
                .for_each(|row| assert_eq!(row.len(), width as usize));
            buffer
                .first()
                .unwrap()
                .iter()
                .for_each(|&n| assert_eq!(n, 35));
            let second_row = buffer.iter().skip(1).next().unwrap();
            second_row.iter().any(|&n| n != 35);
            assert_ne!(previous, buffer);
            previous = buffer.clone();
        }
    }
}

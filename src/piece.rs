use std::fmt;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
    pub color: u32,
}

impl Piece {
    pub fn new(width: usize, height: usize, data: Vec<bool>, color: u32) -> Self {
        Piece {
            width,
            height,
            data,
            color,
        }
    }

    pub fn load(color: u32) -> Result<Self, io::Error> {
        let f = File::open(format!("data/{}.piece", color))?;

        let mut data = vec![];
        let mut width = 0;
        let mut height = 1;
        let mut i = 0;

        for c in f.bytes().map(|b| b.unwrap() as char) {
            if c == '\n' {
                width = i;
                height += 1;
                i = 0;
            } else {
                data.push(c != ' ');
            }
            i += 1;
        }

        Ok(Piece::new(width - 1, height, data, color))
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        unsafe { *self.data.get_unchecked(self.width * i + j) }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = std::char::from_digit(self.color, 10).unwrap();
        for i in 0..self.height {
            for j in 0..self.width {
                f.write_char(if self.get(i, j) { c } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

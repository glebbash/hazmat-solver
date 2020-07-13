use std::fmt;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::prelude::*;

//type Data = Vec<bool>;
type Data = Vec<Vec<bool>>;

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub data: Data,
    pub color: u32,
}

impl Piece {
    pub fn new(width: usize, height: usize, data: Data, color: u32) -> Self {
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
        let mut row = vec![];

        for c in f.bytes().map(|b| b.unwrap() as char) {
            if c == '\n' {
                data.push(row);
                row = vec![];
            } else {
                row.push(c != ' ');
            }
        }
        data.push(row);

        Ok(Piece::new(data[0].len(), data.len(), data, color))
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        unsafe { *self.data.get_unchecked(i).get_unchecked(j) }
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

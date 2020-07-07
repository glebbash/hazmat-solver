use std::fmt;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<bool>>,
    pub color: u32,
}

impl Piece {
    pub fn new(width: usize, height: usize, data: Vec<Vec<bool>>, color: u32) -> Self {
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
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = std::char::from_digit(self.color, 10).unwrap();
        for row in self.data.iter() {
            for b in row {
                f.write_char(if *b { c } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

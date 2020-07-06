use crate::piece::Piece;

pub const SIZE: usize = 24;

pub type Data = [[u32; SIZE]; SIZE];

pub struct Board {
    pub data: Data,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: [[0; SIZE]; SIZE],
        }
    }

    fn set(&mut self, i: usize, j: usize, color: u32) -> bool {
        if self.data[i][j] != 0 {
            false
        } else {
            self.data[i][j] = color;
            true
        }
    }

    pub fn put(&mut self, piece: &Piece, pos: (usize, usize), state: u8) -> bool {
        if state < 4 {
            if pos.0 + piece.height >= SIZE || pos.1 + piece.width >= SIZE {
                return false;
            }
            let i_range: Box<dyn Iterator<Item = _>> = if state == 0 || state == 1 {
                Box::new((0..piece.height).zip(0..piece.height))
            } else {
                Box::new((0..piece.height).zip((0..piece.height).rev()))
            };
            for (i, piece_i) in i_range {
                let j_range: Box<dyn Iterator<Item = _>> = if state == 0 || state == 3 {
                    Box::new((0..piece.width).zip(0..piece.width))
                } else {
                    Box::new((0..piece.width).zip((0..piece.width).rev()))
                };
                for (j, piece_j) in j_range {
                    if piece.data[piece_i][piece_j] {
                        if !self.set(i + pos.0, j + pos.1, piece.color) {
                            self.remove(piece.color);
                            return false
                        }
                    }
                }
            }
        } else {
            if pos.0 + piece.width >= SIZE || pos.1 + piece.height >= SIZE {
                return false;
            }
            let i_range: Box<dyn Iterator<Item = _>> = if state == 4 || state == 5 {
                Box::new((0..piece.height).zip(0..piece.height))
            } else {
                Box::new((0..piece.height).zip((0..piece.height).rev()))
            };
            for (i, piece_i) in i_range {
                let j_range: Box<dyn Iterator<Item = _>> = if state == 4 || state == 6 {
                    Box::new((0..piece.width).zip(0..piece.width))
                } else {
                    Box::new((0..piece.width).zip((0..piece.width).rev()))
                };
                for (j, piece_j) in j_range {
                    if piece.data[piece_i][piece_j] {
                        if !self.set(j + pos.0, i + pos.1, piece.color) {
                            self.remove(piece.color);
                            return false
                        }
                    }
                }
            }
        }
        true
    }
    pub fn remove(&mut self, color: u32) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.data[i][j] == color {
                    self.data[i][j] = 0;
                }
            }
        }
    }
    pub fn print(&self) {
        for row in &self.data {
            for piece in row {
                print!(
                    "{}",
                    if *piece == 0 {
                        '.'
                    } else {
                        std::char::from_digit(*piece, 10).unwrap()
                    }
                )
            }
            print!("\n")
        }
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = u32;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.data[pos.0][pos.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self.data[pos.0][pos.1]
    }
}

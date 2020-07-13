use crate::piece::Piece;
use crate::{get, set};
use ansi_term::Colour::*;
use std::fmt;

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

    pub fn put(
        &mut self,
        piece: &Piece,
        pos_i: usize,
        pos_j: usize,
        max_i: usize,
        max_j: usize,
        flip_i: bool,
        flip_j: bool,
        rot90: bool,
    ) -> bool {
        if rot90 {
            if flip_i {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, max_j - j) {
                                if self.get(j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, max_j - j) {
                                self.set(j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, j) {
                                if self.get(j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, j) {
                                self.set(j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                }
            } else {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, max_j - j) {
                                if self.get(j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, max_j - j) {
                                self.set(j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, j) {
                                if self.get(j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, j) {
                                self.set(j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                }
            }
        } else {
            if flip_i {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, max_j - j) {
                                if self.get(i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, max_j - j) {
                                self.set(i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, j) {
                                if self.get(i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(max_i - i, j) {
                                self.set(i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                }
            } else {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, max_j - j) {
                                if self.get(i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, max_j - j) {
                                self.set(i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, j) {
                                if self.get(i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if piece.get(i, j) {
                                self.set(i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn get(&self, i: usize, j: usize) -> u32 {
        get!(self.data, i, j)
    }

    pub fn set(&mut self, i: usize, j: usize, color: u32) {
        set!(self.data, i, j, color);
    }
}

fn empty() -> String {
    " ".to_string()
    //Black.on(Black).paint(".").to_string()
}

fn display(color: u32) -> String {
    //std::char::from_digit(color, 10).unwrap().to_string()
    match color {
        1 => Blue.on(Blue).paint(" ").to_string(),
        _ => " ".to_string(),
    }
}

impl fmt::Display for Board {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for piece in row {
                write!(
                    out,
                    "{}",
                    if *piece == 0 {
                        empty()
                    } else {
                        display(*piece)
                    }
                )?;
            }
            write!(out, "\n")?;
        }
        Ok(())
    }
}

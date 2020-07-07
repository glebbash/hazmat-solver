use crate::piece::Piece;
use crate::{get, set};

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

    pub fn put(&mut self, piece: &Piece, pos_i: usize, pos_j: usize, state: u8) -> bool {
        let rot90 = state > 3;
        let max_i = piece.height - 1;
        let max_j = piece.width - 1;
        let flip_i = match state {
            0 | 1 | 4 | 5 => false,
            _ => true,
        };
        let flip_j = match state {
            0 | 3 | 4 | 6 => false,
            _ => true,
        };
        if rot90 {
            if flip_i {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, max_j - j) {
                                if get!(self.data, j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, max_j - j) {
                                set!(self.data, j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, j) {
                                if get!(self.data, j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, j) {
                                set!(self.data, j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                }
            } else {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, max_j - j) {
                                if get!(self.data, j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, max_j - j) {
                                set!(self.data, j + pos_i, i + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, j) {
                                if get!(self.data, j + pos_i, i + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, j) {
                                set!(self.data, j + pos_i, i + pos_j, piece.color);
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
                            if get!(piece.data, max_i - i, max_j - j) {
                                if get!(self.data, i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, max_j - j) {
                                set!(self.data, i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, j) {
                                if get!(self.data, i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, max_i - i, j) {
                                set!(self.data, i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                }
            } else {
                if flip_j {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, max_j - j) {
                                if get!(self.data, i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, max_j - j) {
                                set!(self.data, i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                } else {
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, j) {
                                if get!(self.data, i + pos_i, j + pos_j) != 0 {
                                    return false;
                                }
                            }
                        }
                    }
                    for i in 0..=max_i {
                        for j in 0..=max_j {
                            if get!(piece.data, i, j) {
                                set!(self.data, i + pos_i, j + pos_j, piece.color);
                            }
                        }
                    }
                }
            }
        }
        // // not optimized
        //
        // // check loop
        // for i in 0..=max_i {
        //     for j in 0..=max_j {
        //         let piece_i = if flip_i { max_i - i } else { i };
        //         let piece_j = if flip_j { max_j - j } else { j };

        //         if get!(piece.data, piece_i, piece_j) {
        //             if rot90 {
        //                 if get!(self.data, j + pos_i, i + pos_j] != 0 {
        //                     return false;
        //                 }
        //             } else if get!(self.data, i + pos_i, j + pos_j] != 0 {
        //                 return false;
        //             }
        //         }
        //     }
        // }
        // // fill loop
        // for i in 0..=max_i {
        //     for j in 0..=max_j {
        //         let piece_i = if flip_i { max_i - i } else { i };
        //         let piece_j = if flip_j { max_j - j } else { j };

        //         if get!(piece.data, piece_i, piece_j) {
        //             if rot90 {
        //                 set!(self.data, j + pos_i, i + pos_j], piece.color);
        //             } else {
        //                 set!(self.data, i + pos_i, j + pos_j, piece.color);
        //             }
        //         }
        //     }
        // }
        true
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

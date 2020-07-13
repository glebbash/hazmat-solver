mod board;
mod get_set;
mod piece;

use board::Board;
use piece::Piece;
use std::time::Instant;
use std::io::Write;
//use std::path::Path;
//use std::fs::File;

fn main() {
    let mut board: Board = Board::new();
    let pieces: Vec<Piece> = (1..=9).map(|i| Piece::load(i).unwrap()).collect();
    let before = Instant::now();
    put_piece(&mut board, &pieces, 0, pieces.len() - 1);
    //let p = &pieces[0];
    //board.put(p, 0, 0, p.height - 1, p.width - 1, false, false, false);

    //let mut out = File::create(Path::new("data/res-c.txt")).unwrap();
    let mut out = std::io::stdout();
    write!(out, "{}", board).unwrap();
    println!("{:?}", before.elapsed());
}

fn put_piece(board: &mut Board, pieces: &[Piece], index: usize, end: usize) -> bool {
    let piece = unsafe { &pieces.get_unchecked(index) };
    let max_i = piece.height - 1;
    let max_j = piece.width - 1;

    for state in 0..1 {
        let rot90 = state > 3;
        let (bottom, right) = if rot90 {
            (max_j, max_i)
        } else {
            (max_i, max_j)
        };
        let flip_i = match state {
            0 | 1 | 4 | 5 => false,
            _ => true,
        };
        let flip_j = match state {
            0 | 3 | 4 | 6 => false,
            _ => true,
        };
        for i in (0..board::SIZE - bottom).step_by(2) {
            for j in (0..board::SIZE - right).step_by(2) {
                if board.put(piece, i, j, max_i, max_j, flip_i, flip_j, rot90) {
                    // if index > 5 {
                    //     println!("{}", board);
                    // }
                    if index == end {
                        return true;
                    }
                    if put_piece(board, pieces, index + 1, end) {
                        return true;
                    } else {
                        remove_piece(board, i, j, i + bottom, j + right, piece.color);
                    }
                }
            }
        }
    }
    false
}

fn remove_piece(board: &mut Board, si: usize, sj: usize, ei: usize, ej: usize, color: u32) {
    for i in si..=ei {
        for j in sj..=ej {
            if get!(board.data, i, j) == color {
                set!(board.data, i, j, 0);
            }
        }
    }
}

// 0..1 all -> 50s
// 0..1 sec -> 40s (apps closed)
// 0..1 sec -> 7s (server)
// 0..2 -> 528s (server)

// 0..2 step 4 -> 400ms
// 0..3 step 4 -> 4s
// 0..4 step 4 -> 24s

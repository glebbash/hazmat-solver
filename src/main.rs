mod board;
mod piece;
mod get_set;

use board::Board;
use piece::Piece;
use std::time::Instant;

fn main() {
    let mut board: Board = Board::new();
    let pieces: Vec<Piece> = (1..=9).map(|i| Piece::load(i).unwrap()).collect();
    let before = Instant::now();
    put_piece(&mut board, &pieces, 0, pieces.len() - 1);
    board.print();
    println!("{:?}", before.elapsed());
}

fn put_piece(board: &mut Board, pieces: &[Piece], index: usize, end: usize) -> bool {
    let piece = unsafe { &pieces.get_unchecked(index) };
    let max_i = piece.height - 1;
    let max_j = piece.width - 1;

    for state in 0..1 {
        let (bottom, right) = if state > 3 {
            (max_j, max_i)
        } else {
            (max_i, max_j)
        };
        for i in (0..board::SIZE - bottom).step_by(2) {
            for j in (0..board::SIZE - right).step_by(2) {
                if board.put(piece, i, j, state) {
                    // if index > 5 {
                    //     board.print();
                    //     println!("{}", index);
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
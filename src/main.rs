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
                    // remove_piece(board, i, j, i + bottom, j + right, piece.color);
                    // return true;
                    // if index > 5 {
                    //     board.print();
                    //     println!("{}", index);
                    // }
                    if index == end {
                        return true;
                    }
                    let put_all = put_piece(board, pieces, index + 1, end);
                    if put_all {
                        return true;
                    } else {
                        //remove_piece(board, i, j, i + bottom, j + right, piece.color);
                        remove_piece_1(board, piece.color);
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

fn remove_piece_1(board: &mut Board, color: u32) {
    for i in 0..board::SIZE {
        for j in 0..board::SIZE {
            if get!(board.data, i, j) == color {
                set!(board.data, i, j, 0);
            }
        }
    }
}

// 0..0 -> 4.1µs        4
// 0..1 -> 168.4216ms   168000
//                      x15
// 0..2 -> 2.5868068s   2600000
//                      x6.5
// 0..3 -> 17.7953124s  17000000
//                      x4.7
// 0..4 -> 80.2822031s  80000000
//                      x1.9
// 0..5 -> 149.868599s  150000000
//                      x2.07
// 0..6 -> 311.1525722s 311000000
//                      x2.2
// 0..7 ->              (684s)
//                      x2.5
// 0..8 -> 617.4269947s 617000000
//
/////////////////////////////////
// v2
//
// 1 -> 2s
//                      x34
// 2 -> 68s
//                      x9
// 3 -> 613s
//                      x3
// 4 ->
//                      x
// 5 ->
//                      x
// 6 ->
//                      x
// 7 ->
//                      x
// 8 -> 24953s

// 0..1    +optimized -> 52s
// 0..1   ++optimized -> 51, 46, 48, 52s
// 0..1  +++optimized -> 44, 42, 42, 50, 49, 50
// 0..1 ++++optimized -> 44s
// 0..1    -optimized -> 60s

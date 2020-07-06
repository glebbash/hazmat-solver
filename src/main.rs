mod board;
mod piece;

use board::Board;
use piece::Piece;

fn main() {
    let mut board: Board = Board::new();
    let pieces: Vec<Piece> = (1..=9).map(|i| Piece::load(i).unwrap()).collect();
    put_piece(&mut board, &pieces, 0);
    board.print();
}

fn put_piece(board: &mut Board, pieces: &Vec<Piece>, index: usize) -> bool {
    let piece = &pieces[index];
    for state in 0..8 {
        for i in 0..board::SIZE {
            for j in 0..board::SIZE {
                if board.put(piece, (i, j), state) {
                    if index == pieces.len() - 1 {
                        return true;
                    }
                    let succ = put_piece(board, pieces, index + 1);
                    if succ {
                        return true;
                    } else {
                        board.remove(piece.color);
                    }
                }
            }
        }
    }
    false
}
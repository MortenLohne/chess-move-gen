use board_game_traits::board::Board;

/// Returns how many moves the move generator finds after after searching `depth` plies deep.
/// This provides confidence that the move generator is correct
pub fn perft<B: Board>(board : &mut B, depth : u16) -> u64
{
    if depth == 0 { 1 }
    else if board.game_result() != None { 0 } else {
        let mut moves = Vec::with_capacity(100);
        board.generate_moves(&mut moves);
        if depth == 1 { moves.len() as u64 } else {
            let mut total_moves = 0;
            for c_move in moves {
                let reverse_move = board.do_move(c_move.clone());
                total_moves += perft(board, depth - 1);
                board.reverse_move(reverse_move);
            }
            total_moves
        }
    }
}

#[cfg(test)]
/// Verifies the perft result of a position against a known answer
pub fn perft_check_answers<B: Board>(board: &mut B, answers: &[u64]) {
    for (depth, &answer) in answers.iter().enumerate() {
        assert_eq!(perft(board, depth as u16), answer);
    }
}
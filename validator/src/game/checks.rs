use super::*;
pub fn is_king_in_check(state: &GameState, color: Color) -> bool {
    let pieces = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
    ];
    for piece in pieces {
        let piece_value = match color {
            Color::White => state.get_piece(Color::Black, piece),
            Color::Black => state.get_piece(Color::White, piece),
        };
    }
    unimplemented!("TODO finish this");
    false
}

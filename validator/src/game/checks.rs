use super::pieces::get_piece_threats_from;
use super::*;

pub fn is_king_in_check(state: &GameState, color: Color) -> Result<bool, ForbiddenMoveError> {
    const ALL_PIECES: [Piece; 6] = Piece::list();
    let king = state.get_piece(color, Piece::King);
    for piece in ALL_PIECES {
        let (piece_value, other_color) = match color {
            Color::White => (state.get_piece(Color::Black, piece), Color::Black),
            Color::Black => (state.get_piece(Color::White, piece), Color::White),
        };
        let pieces = set_bits(piece_value);
        for single_piece in pieces {
            let single_piece_moves = get_piece_threats_from(
                single_piece,
                piece,
                other_color,
                state.get_color_pieces(other_color),
                state.get_color_pieces(color),
            )?;
            if single_piece_moves & king != 0 {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

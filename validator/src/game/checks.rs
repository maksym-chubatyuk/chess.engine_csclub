use super::pieces::get_piece_threats_from;
use super::*;

pub fn is_king_in_check(state: &GameState, color: Color) -> Result<bool, ForbiddenMoveError> {
    let all_pieces = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
    let king = state.get_piece(color, Piece::King);
    for piece in all_pieces {
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

/// Returns an iterator of u64s that correspond to each set bit
/// of the argument u64
fn set_bits(mut bits: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if bits == 0 {
            return None;
        }

        let bit = bits & bits.wrapping_neg();
        bits &= bits - 1;
        Some(bit)
    })
}

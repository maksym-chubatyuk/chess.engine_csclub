use super::helpers::*;

// TODO: Promotion
pub fn get_pawn_moves_from(
    from: u64,
    color: Option<Color>,
    team_pieces: u64,
    enemy_pieces: u64,
) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("Pawn"));
    }
    let empty_squares = !(enemy_pieces | team_pieces);

    let color = color.ok_or_else(|| ForbiddenMoveError("Need to provide color for pawn moves"))?;

    let mut result = 0u64;
    // Pawns are weird because only one direction so we need diff logic for white and black
    match color {
        Color::White => {
            const THIRD_RANK_MASK: u64 = 0x0000000000FF0000;
            result |= (from << 8) & empty_squares;
            // We use result and not shifting the initial position by 16 because that would
            // not detect if there was something in the way
            result |= ((result & THIRD_RANK_MASK) << 8) & empty_squares;
        }
        Color::Black => {
            const SIXTH_RANK_MASK: u64 = 0x0000FF0000000000;
            result |= (from >> 8) & empty_squares;
            result |= ((result & SIXTH_RANK_MASK) >> 8) & empty_squares;
        }
    };
    result |= get_pawn_threats_from(from, color) & enemy_pieces;
    Ok(result)
}

fn get_pawn_threats_from(pawn: u64, color: Color) -> u64 {
    let mut result = 0u64;
    // TODO: En passant
    match color {
        Color::White => {
            let attack_left = (pawn << 7) & NOT_H_FILE;
            let attack_right = (pawn << 9) & NOT_A_FILE;
            result |= attack_left | attack_right;
        }
        Color::Black => {
            let attack_left = (pawn >> 9) & NOT_H_FILE;
            let attack_right = (pawn >> 7) & NOT_A_FILE;
            result |= attack_left | attack_right;
        }
    };
    result
}

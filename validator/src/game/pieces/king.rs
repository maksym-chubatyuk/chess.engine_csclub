use super::*;

/// Produces the map of all potential king moves from a square
/// NOTE: This **DOES NOT** take into account any threats from enemy pieces
/// It **ONLY** produces the map of every potential move
pub fn get_king_moves_from(from: u64, team_pieces: u64) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("King"));
    }

    let east = (from & NOT_H_FILE) << 1;
    let west = (from & NOT_A_FILE) >> 1;
    let horizontal = from | east | west;

    let moves = (horizontal << 8) | (horizontal >> 8) | east | west;

    Ok(moves & !team_pieces)
}

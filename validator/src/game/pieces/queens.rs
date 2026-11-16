use super::bishops::get_bishop_moves_from;
use super::rooks::get_rook_moves_from;
use super::*;

pub fn get_queen_moves_from(
    from: u64,
    team_pieces: u64,
    enemy_pieces: u64,
) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("Queen"));
    }
    Ok(get_rook_moves_from(from, team_pieces, enemy_pieces)?
        | get_bishop_moves_from(from, team_pieces, enemy_pieces)?)
}

mod bishops;
mod king;
mod knights;
mod pawns;
mod queens;
mod rooks;
pub use super::helpers::*;

/// Produces the map of all potential moves from a square for a piece of a certain color.
/// **NOTE**: This DOES NOT take into account forced moves, checks, pins, etc...
/// it only returns the moves that a piece *could* make.
pub fn get_piece_moves_from(
    from: u64,
    piece: Piece,
    color: Color,
    team_pieces: u64,
    enemy_pieces: u64,
) -> Result<u64, ForbiddenMoveError> {
    match piece {
        Piece::Pawn => pawns::get_pawn_moves_from(from, color, team_pieces, enemy_pieces),
        Piece::Knight => knights::get_knight_moves_from(from, team_pieces),
        Piece::Bishop => bishops::get_bishop_moves_from(from, team_pieces, enemy_pieces),
        Piece::Rook => rooks::get_rook_moves_from(from, team_pieces, enemy_pieces),
        Piece::Queen => queens::get_queen_moves_from(from, team_pieces, enemy_pieces),
        Piece::King => king::get_king_moves_from(from, team_pieces),
    }
}

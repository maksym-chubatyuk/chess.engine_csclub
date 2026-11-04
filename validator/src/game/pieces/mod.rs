mod bishops;
mod knights;
mod pawns;
mod rooks;
// TODO: Finish separation from gamestate
pub use super::*;

pub type GetPieceMovesFrom =
    fn(from: u64, color: Option<Color>, team_pieces: u64, enemy_pieces: u64) -> u64;

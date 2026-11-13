pub mod bishops;
pub mod king;
pub mod knights;
pub mod pawns;
pub mod rooks;
pub use super::helpers::*;

pub type GetPieceMovesFrom =
    fn(from: u64, color: Option<Color>, team_pieces: u64, enemy_pieces: u64) -> u64;

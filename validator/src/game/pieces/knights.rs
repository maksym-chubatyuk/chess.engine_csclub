use super::helpers::*;

// TODO: Could just shift a mask by index, maybe more efficient?
pub fn get_knight_moves_from(from: u64, team_pieces: u64) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("Knight"));
    };

    let mut result = 0u64;
    result |= from << 2 * 8 + 1;
    result |= from << 2 * 8 - 1;
    result |= from << 1 * 8 + 2;
    result |= from << 1 * 8 - 2;
    result |= from >> 2 * 8 + 1;
    result |= from >> 2 * 8 - 1;
    result |= from >> 1 * 8 + 2;
    result |= from >> 1 * 8 - 2;

    result &= !team_pieces;

    // Prevent horizontal wrapping
    const NOT_FIRST_TWO_FILES: u64 = 0xFCFCFCFCFCFCFCFC;
    const NOT_LAST_TWO_FILES: u64 = 0x3F3F3F3F3F3F3F3F;
    // !NOT makes a mask of only the last two files
    if from & !NOT_FIRST_TWO_FILES != 0 {
        result &= NOT_LAST_TWO_FILES
    } else if from & !NOT_LAST_TWO_FILES != 0 {
        result &= NOT_FIRST_TWO_FILES
    }

    Ok(result)
}

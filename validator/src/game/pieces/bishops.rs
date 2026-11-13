use super::*;

pub fn get_bishop_moves_from(
    from: u64,
    team_pieces: u64,
    enemy_pieces: u64,
) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("Bishop"));
    }

    let mut result = 0u64;
    let mut pointer = from;

    // TODO : make a helper function for sliding pieces because this feels too repetitive
    // Up and left
    loop {
        pointer <<= 7;
        if pointer & (NOT_H_FILE & NOT_FIRST_RANK) == 0 {
            break;
        }
        if pointer & enemy_pieces != 0 {
            result |= pointer;
            break;
        } else if pointer & team_pieces != 0 {
            break;
        }
        result |= pointer;
    }
    pointer = from;
    // Up and right
    loop {
        pointer <<= 9;
        if pointer & (NOT_A_FILE & NOT_FIRST_RANK) == 0 {
            break;
        };
        if pointer & enemy_pieces != 0 {
            result |= pointer;
            break;
        } else if pointer & team_pieces != 0 {
            break;
        }
        result |= pointer;
    }
    pointer = from;
    // Down and left
    loop {
        pointer >>= 9;
        if pointer & (NOT_H_FILE & NOT_LAST_RANK) == 0 {
            break;
        }
        if pointer & enemy_pieces != 0 {
            result |= pointer;
            break;
        } else if pointer & team_pieces != 0 {
            break;
        }
        result |= pointer;
    }
    pointer = from;
    // Down and right
    loop {
        pointer >>= 7;
        if pointer & (NOT_A_FILE & NOT_LAST_RANK) == 0 {
            break;
        };
        if pointer & enemy_pieces != 0 {
            result |= pointer;
            break;
        } else if pointer & team_pieces != 0 {
            break;
        }
        result |= pointer;
    }

    Ok(result)
}

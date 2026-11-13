use super::*;

//TODO: This is broken around the edges
pub fn get_rook_moves_from(
    from: u64,
    team_pieces: u64,
    enemy_pieces: u64,
) -> Result<u64, ForbiddenMoveError> {
    if from.count_ones() != 1 {
        return Err(ForbiddenMoveError("Rook"));
    }

    let mut result = 0u64;
    let mut pointer = from;

    // Right
    loop {
        pointer <<= 1;
        if pointer & NOT_H_FILE == 0 {
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

    // Left
    pointer = from;
    loop {
        pointer >>= 1;
        if pointer & NOT_A_FILE == 0 {
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

    // Up
    pointer = from;
    loop {
        pointer <<= 8;
        if pointer & NOT_FIRST_RANK == 0 {
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

    // Down
    pointer = from;
    loop {
        pointer >>= 8;
        if pointer & NOT_LAST_RANK == 0 {
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

    Ok(result)
}

"""Tests for chess engine encoding."""

import chess
import numpy as np
import pytest

from engine.encoding import (
    NUM_PLANES,
    POLICY_SIZE,
    encode_board,
    move_to_index,
    index_to_move,
    legal_mask,
    legal_moves_supported,
)


def test_round_trip_random_games():
    """Play 500 random games, verify round-trip at every position."""
    rng = np.random.default_rng(seed=42)
    num_games = 500
    max_plies = 200

    for _ in range(num_games):
        board = chess.Board()
        for _ in range(max_plies):
            supported = legal_moves_supported(board)
            for move in supported:
                idx = move_to_index(move, board)
                recovered = index_to_move(idx, board)
                assert recovered == move, f"Round-trip failed: {move} -> {idx} -> {recovered}"
            if not supported or board.is_game_over():
                break
            move = rng.choice(supported)
            board.push(move)


def test_no_index_collisions():
    """At every position, all supported legal moves map to distinct indices."""
    rng = np.random.default_rng(seed=123)
    num_games = 500
    max_plies = 200

    for _ in range(num_games):
        board = chess.Board()
        for _ in range(max_plies):
            supported = legal_moves_supported(board)
            indices = [move_to_index(m, board) for m in supported]
            assert len(indices) == len(set(indices)), "Index collision detected"
            if not supported or board.is_game_over():
                break
            move = rng.choice(supported)
            board.push(move)


def test_mask_matches_moves():
    """legal_mask sum equals len(legal_moves_supported), mask True exactly at those indices."""
    rng = np.random.default_rng(seed=456)
    num_games = 500
    max_plies = 200

    for _ in range(num_games):
        board = chess.Board()
        for _ in range(max_plies):
            supported = legal_moves_supported(board)
            mask = legal_mask(board)
            assert mask.sum() == len(supported), f"Mask sum {mask.sum()} != supported count {len(supported)}"
            for move in supported:
                idx = move_to_index(move, board)
                assert mask[idx], f"Move {move} at index {idx} not in mask"
            if not supported or board.is_game_over():
                break
            move = rng.choice(supported)
            board.push(move)


def test_shapes_and_dtypes():
    """Verify array shapes, dtypes, and value ranges."""
    board = chess.Board()

    encoded = encode_board(board)
    assert encoded.shape == (NUM_PLANES, 8, 8), f"Shape {encoded.shape} != ({NUM_PLANES}, 8, 8)"
    assert encoded.dtype == np.float32, f"dtype {encoded.dtype} != float32"
    assert np.all(encoded >= 0.0) and np.all(encoded <= 1.0), "Values outside [0, 1]"

    mask = legal_mask(board)
    assert mask.shape == (POLICY_SIZE,), f"Shape {mask.shape} != ({POLICY_SIZE},)"
    assert mask.dtype == bool, f"dtype {mask.dtype} != bool"


def test_start_position():
    """Verify specific planes in the start position."""
    board = chess.Board()
    encoded = encode_board(board)

    # Plane 0: own pawns on rank 1 (index 1)
    assert np.sum(encoded[0, 1, :]) == 8, "Plane 0 should have 8 pawns on rank 1"
    # Plane 5: own king at [0, 4] (e1)
    assert encoded[5, 0, 4] == 1.0, "Plane 5 should have king at [0, 4]"
    # Planes 12-15: castling rights all 1.0
    for plane_idx in range(12, 16):
        assert np.all(encoded[plane_idx, :, :] == 1.0), f"Plane {plane_idx} should be all 1.0"
    # Plane 16: en passant all zeros
    assert np.all(encoded[16, :, :] == 0.0), "Plane 16 should be all zeros"
    # Plane 17: halfmove clock all zeros
    assert np.all(encoded[17, :, :] == 0.0), "Plane 17 should be all zeros"


def test_perspective_symmetry():
    """encode_board(board) == encode_board(board.mirror()) for 200 random positions."""
    rng = np.random.default_rng(seed=789)

    for _ in range(200):
        board = chess.Board()
        # Play some random moves to get a random position
        num_moves = rng.integers(0, 50)
        for _ in range(num_moves):
            supported = legal_moves_supported(board)
            if not supported or board.is_game_over():
                break
            move = rng.choice(supported)
            board.push(move)

        mirrored = board.mirror()

        # Encodings should be identical
        enc1 = encode_board(board)
        enc2 = encode_board(mirrored)
        assert np.array_equal(enc1, enc2), "encode_board not symmetric under mirror"

        # Move indices should match for corresponding moves
        supported1 = legal_moves_supported(board)
        supported2 = legal_moves_supported(mirrored)

        # Build mapping from original move to mirrored move
        for move in supported1:
            mirrored_move = chess.Move(
                chess.square_mirror(move.from_square),
                chess.square_mirror(move.to_square),
                move.promotion,
            )
            if mirrored_move in supported2:
                idx1 = move_to_index(move, board)
                idx2 = move_to_index(mirrored_move, mirrored)
                assert idx1 == idx2, f"Move index not symmetric: {move} vs {mirrored_move}"


# Hand-built cases
def test_castling_white():
    """Test White castling moves."""
    # Kingside
    board = chess.Board("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1")
    move = chess.Move.from_uci("e1g1")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"White O-O: {move} -> {idx} -> {recovered}"

    # Queenside
    move = chess.Move.from_uci("e1c1")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"White O-O-O: {move} -> {idx} -> {recovered}"


def test_castling_black():
    """Test Black castling moves (check mirrored indices)."""
    board = chess.Board("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1")
    # Kingside: e8g8 (from Black's perspective, mirrored to e1g1)
    move = chess.Move.from_uci("e8g8")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"Black O-O: {move} -> {idx} -> {recovered}"

    # Queenside: e8c8
    move = chess.Move.from_uci("e8c8")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"Black O-O-O: {move} -> {idx} -> {recovered}"


def test_en_passant():
    """Test en passant captures and plane 16."""
    # White en passant: black pawn just moved e7e5, white pawn on d5 captures d5e6
    board = chess.Board("rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3")
    move = chess.Move.from_uci("d5e6")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"White e.p.: {move} -> {idx} -> {recovered}"

    encoded = encode_board(board)
    # e6 is square 44, rank=5, file=4. White turn, no mirror needed.
    assert encoded[16, 5, 4] == 1.0, "Plane 16 should mark e6 for White e.p."

    # Black en passant: white pawn just moved e2e4, black pawn on d4 captures d4e3
    board = chess.Board("rnbqkbnr/pppp1ppp/8/8/3pP3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 3")
    move = chess.Move.from_uci("d4e3")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"Black e.p.: {move} -> {idx} -> {recovered}"

    encoded = encode_board(board)
    # e3 is square 20 (rank 2). Black turn, board is mirrored.
    # square_mirror(20) = 44 (e6), rank=5, file=4
    assert encoded[16, 5, 4] == 1.0, "Plane 16 should mark mirrored e.p. square for Black"


def test_promotion():
    """Test queen promotions."""
    # White promotion
    board = chess.Board("8/P7/8/8/8/8/8/8 w - - 0 1")
    move = chess.Move.from_uci("a7a8q")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"White promotion: {move} -> {idx} -> {recovered}"
    assert recovered.promotion == chess.QUEEN

    # Black promotion
    board = chess.Board("8/8/8/8/8/8/p7/8 b - - 0 1")
    move = chess.Move.from_uci("a2a1q")
    idx = move_to_index(move, board)
    recovered = index_to_move(idx, board)
    assert recovered == move, f"Black promotion: {move} -> {idx} -> {recovered}"
    assert recovered.promotion == chess.QUEEN


def test_underpromotion_excluded():
    """Underpromotions are NOT in legal_mask or legal_moves_supported."""
    board = chess.Board("8/P7/8/8/8/8/8/8 w - - 0 1")
    underpromo = chess.Move.from_uci("a7a8n")  # Knight promotion
    queen_promo = chess.Move.from_uci("a7a8q")  # Queen promotion

    mask = legal_mask(board)
    supported = legal_moves_supported(board)

    assert underpromo not in supported, "Underpromotion should not be in supported moves"

    # Underpromotion and queen promotion share same index (promotions not encoded)
    assert move_to_index(underpromo, board) == move_to_index(queen_promo, board)

    # Since queen promotion is legal, its index is True in mask
    idx = move_to_index(underpromo, board)
    assert mask[idx], f"Queen promotion index {idx} should be True in mask"


def test_input_not_mutated():
    """board.fen() is identical before and after encode_board."""
    board = chess.Board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
    fen_before = board.fen()
    encode_board(board)
    fen_after = board.fen()
    assert fen_before == fen_after, "encode_board mutated the input board"


def test_constants():
    """Verify constants are defined correctly."""
    assert NUM_PLANES == 18
    assert POLICY_SIZE == 4096


def test_index_to_move_invalid():
    """index_to_move raises ValueError for out-of-range indices."""
    board = chess.Board()
    with pytest.raises(ValueError):
        index_to_move(-1, board)
    with pytest.raises(ValueError):
        index_to_move(4096, board)
    with pytest.raises(ValueError):
        index_to_move(5000, board)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
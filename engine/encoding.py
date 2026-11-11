"""Chess position and move encoding to numeric form."""

import chess
import numpy as np

NUM_PLANES = 18
POLICY_SIZE = 4096


def encode_board(board: chess.Board) -> np.ndarray:
    """
    Encode a chess board as an 18-plane 8x8 float32 array from the side-to-move's perspective.

    Args:
        board: The chess board to encode.

    Returns:
        np.ndarray of shape (18, 8, 8) and dtype float32.
        Plane 0-5: own pawn, knight, bishop, rook, queen, king
        Plane 6-11: opponent pawn, knight, bishop, rook, queen, king
        Plane 12: own kingside castling right
        Plane 13: own queenside castling right
        Plane 14: opponent kingside castling right
        Plane 15: opponent queenside castling right
        Plane 16: en passant target square
        Plane 17: halfmove clock normalized
    """
    if board.turn == chess.BLACK:
        board = board.mirror()

    planes = np.zeros((NUM_PLANES, 8, 8), dtype=np.float32)

    piece_types = [
        chess.PAWN,
        chess.KNIGHT,
        chess.BISHOP,
        chess.ROOK,
        chess.QUEEN,
        chess.KING,
    ]

    for plane_offset, piece_type in enumerate(piece_types):
        own_pieces = board.pieces(piece_type, chess.WHITE)
        for square in own_pieces:
            rank, file = divmod(square, 8)
            planes[plane_offset, rank, file] = 1.0

        opp_pieces = board.pieces(piece_type, chess.BLACK)
        for square in opp_pieces:
            rank, file = divmod(square, 8)
            planes[plane_offset + 6, rank, file] = 1.0

    if board.has_kingside_castling_rights(chess.WHITE):
        planes[12, :, :] = 1.0
    if board.has_queenside_castling_rights(chess.WHITE):
        planes[13, :, :] = 1.0
    if board.has_kingside_castling_rights(chess.BLACK):
        planes[14, :, :] = 1.0
    if board.has_queenside_castling_rights(chess.BLACK):
        planes[15, :, :] = 1.0

    if board.ep_square is not None:
        rank, file = divmod(board.ep_square, 8)
        planes[16, rank, file] = 1.0

    planes[17, :, :] = min(board.halfmove_clock, 100) / 100.0

    return planes


def move_to_index(move: chess.Move, board: chess.Board) -> int:
    """
    Convert a chess move to a policy index (from_square * 64 + to_square).

    Args:
        move: The chess move to encode.
        board: The board the move is made on (for perspective).

    Returns:
        int in [0, 4096).
    """
    from_square = move.from_square
    to_square = move.to_square

    if board.turn == chess.BLACK:
        from_square = chess.square_mirror(from_square)
        to_square = chess.square_mirror(to_square)

    return from_square * 64 + to_square


def index_to_move(idx: int, board: chess.Board) -> chess.Move:
    """
    Convert a policy index to a chess move.

    Args:
        idx: Policy index in [0, 4096).
        board: The board to convert the move for (for perspective and promotion).

    Returns:
        chess.Move object.

    Raises:
        ValueError: If idx is outside [0, 4096).
    """
    if not 0 <= idx < POLICY_SIZE:
        raise ValueError(f"Index {idx} is outside [0, {POLICY_SIZE})")

    from_square = idx // 64
    to_square = idx % 64

    if board.turn == chess.BLACK:
        from_square = chess.square_mirror(from_square)
        to_square = chess.square_mirror(to_square)

    promotion = None
    if (
        chess.square_rank(to_square) in (0, 7)
        and board.piece_at(from_square) == chess.Piece(chess.PAWN, board.turn)
        and chess.Move(from_square, to_square, promotion=chess.QUEEN) in board.legal_moves
    ):
        promotion = chess.QUEEN

    return chess.Move(from_square, to_square, promotion=promotion)


def legal_mask(board: chess.Board) -> np.ndarray:
    """
    Return a boolean mask of legal moves (excluding underpromotions).

    Args:
        board: The chess board to generate moves for.

    Returns:
        np.ndarray of shape (4096,), dtype bool. True at indices of legal moves
        with promotion None or QUEEN.
    """
    mask = np.zeros(POLICY_SIZE, dtype=bool)

    for move in board.legal_moves:
        if move.promotion is None or move.promotion == chess.QUEEN:
            idx = move_to_index(move, board)
            mask[idx] = True

    return mask


def legal_moves_supported(board: chess.Board) -> list[chess.Move]:
    """
    Return legal moves with no underpromotions.

    Args:
        board: The chess board to generate moves for.

    Returns:
        list of chess.Move objects (no underpromotions).
    """
    return [
        move
        for move in board.legal_moves
        if move.promotion is None or move.promotion == chess.QUEEN
    ]

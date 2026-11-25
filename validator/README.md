# The move validation engine
This library has been written in Rust for efficient bitboard representation.

## Usage: 
First, install the library:

```bash
pip install --upgrade --no-index --find-links=./validator/wheels validator
```

Then, to use:
```python
  from validator import State
  state = State() # Creates a new chess game at default position
  moves = state.get_legal_moves()
  status = state.move_piece("e2e4") # Pushes e2 pawn forward two squares, returns whether it was successful
  state.print()
```

Output:

```text
  +-----------------+
8 | r n b q k b n r |
7 | p p p p p p p p |
6 | • • • • • • • • |
5 | • • • • • • • • |
4 | • • • • P • • • |
3 | • • • • • • • • |
2 | P P P P • P P P |
1 | R N B Q K B N R |
  +-----------------+
    a b c d e f g h
Turn: Black
```


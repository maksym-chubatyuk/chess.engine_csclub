# The move validation engine
This library has been written in Rust for efficient bitboard representation.

## Usage: 
First, install the library:

```bash
pip install --upgrade --no-index --find-links=./validator/wheels validator
```

Then, to use
```python
  from validator import State
  state = State() # Creates a new chess game at default position
  moves = state.get_legal_moves()
```

Currently, the library does not support moving pieces and thus only gives the
legal moves from the starting position.

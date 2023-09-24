# Rust Chess library

## String representation of board

| Piece  | White | Black |
| ------ | ----- | ----- |
| King   | K     | k     |
| Queen  | Q     | q     |
| Bishop | B     | b     |
| Knight | N     | n     |
| Rook   | R     | r     |
| Pawn   | P     | p     |
| Empty  | .     | .

### Initial board state

rnbqkbnrpppppppp................................PPPPPPPPRNBQKBNR

## Chess API

* New game
* Get move list
* Make move
* Get state

## TODO

Change `Coord` to add a `get_x` and `get_y` instead of having to use 
`get_x_and_y` and index the result

Add stalemate logic
# DOING
* GUI in OpenGL

# BACKLOG
* Error propagation
* AI making legal moves
* Add how to play
* Save / export (FEN) positions
* GUI in Vulkan

# Done
- [x] Added MIT license
- [x] Castle availability, en-passant fields and clock counts
- [x] Checkmate and stalemate
- [x] Implement en-passant and castling in the move execution
- [x] Assert that the king doesn't move through check when castling
- [x] Assert that the move will not result in the player being in check
- [x] Logic to discern whether a player is in check in a given position
- [x] Loading FEN strings
- [x] Handle en-passant and castling when checking legal moves
- [x] Assert that there's no own piece on the field
- [x] Basic logic to undo a move (not yet connected to UI)
- [x] Assert that the way to the target field is not blocked
- [x] Assert that the selected piece can in principle reach the target field
- [x] Assert that a proper piece was selected (no empty field, right color)
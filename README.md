# DOING
* Check move validity

# TODO
- [x] Assert that a proper piece was selected (no empty field, right color)
- [x] Assert that the selected piece can in principle reach the target field
- [x] Handle en-passant and castling in the move check
- [ ] Implement en-passant and castling in the move execution
- [ ] Unit tests for target reach check for pieces other than pawns
- [ ] Assert that player's piece is not yet on the field
- [ ] Assert that the way to the target field is not blocked
- [ ] Assert that the move will not result in the player being in check
- [ ] Assert that the king doesn't move through check when castling

# BACKLOG
* Checkmate and stalemate
* AI making legal moves
* Import / export positions
* Don't panic on legal moves!
* OpenGL support
* Vulkan support
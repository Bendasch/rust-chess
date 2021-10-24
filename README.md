# DOING
* Assertion that the inserted move is legal

# TODO
- [x] Assert that a proper piece was selected (no empty field, right color)
- [x] Unit tests for target reach check for pieces other than pawns
- [x] Assert that the selected piece can in principle reach the target field
- [x] Handle en-passant and castling in the move check
- [x] Assert that player's piece is not yet on the field
- [x] Loading FEN strings
- [x] Test the FEN loading
- [ ] Assert that the way to the target field is not blocked
- [ ] Assert that the move will not result in the player being in check
- [ ] Assert that the king doesn't move through check when castling
- [ ] Implement en-passant and castling in the move execution

# BACKLOG
* Checkmate and stalemate
* AI making legal moves
* Don't panic on illegal moves!
* Graceful exit
* Save / export (FEN) positions
* OpenGL support
* Vulkan support
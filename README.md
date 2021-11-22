![gui](gui.gif)

# Requirements
- Windows 2000 Professional or later
- OpenGL version 3.3 or higher

# How to play
## GUI
```
cargo run gui
```
## CLI
```
cargo run cli
```
The game can be played by the user by entering a move in the form:
    <start row index><start column index><target row index><target column index>

For example, the starting move "e4" would be entered as "2545". In the future parsing of moves in more intuitive
notation may be supported.

# To do
- [ ] Proper error propagation to and handling in UI
- [ ] Revert moves in UI
- [ ] Save / export FEN strings
- [ ] AI making legal moves
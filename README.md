# sudoku
A basic sudoku solver built in Rust

# Info

The aim of this little tool is to grow into a powerful sudoku solver, capable of all basic and advanced solving techniques used by humans. These techniques will be added one by one after the basic sudoku engine works and reliably manages user input.

# TODO

This list in very much incomplete, but provides a rough outline of what is going to happen over time

- Finish implemenation of the playing field itself, including getters for all the cells and houses
- Provide the user with methods to input a sudoku and to display its current state
- Build first basic solving algorithms: naked singles and hidden singles
- Reliably eliminate a solved digit from all intersecting cells
- Basic "engine" methods. Make the solver choose its own techniques while traversing the board
- Intermediate techniques: naked pairs, triplets, quads, hidden pairs, triplets, quads, omission
- basic GUI input
- Advanced techniques: X Wing, Swordfish, XY Wing, Unique Rectangle, etc.

# Building

Build and run the software using 
```
cargo run
```

Unit tests can be run with
```
cargo test
```

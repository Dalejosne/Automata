# Game of life crate

This crate allow you tu create an play the famous "game of life" of John Conway.
## Presentation
***But what is the game of life ?***\
It is an automata invented by John Conway in the 70s.\
It is composed of an infinite grid (in practice it is finite), of which cell can be in two state : dead or alive.\
The neigbourhood of a cell is the 8 cells which around it (e.g bottom, top, left, right and diagonals).\
If an alive cell has two or three neighboors who are alive, it becomes alive. Else, it dies.\
If a dead cell has three neighboors who are alive, it becomes alive. Else, it stays dead.\
So what you have to do is just create an automata, set some cells (to meke them alive) and to see how it evoluates !\
For more information about conway game of life, have a look to [this](https://www.conwaylife.com)
***Have fun !***

## Install
To install it as a dependency, just add this line to your Cargo.toml :
```toml
[dependencies]
game_of_life = "0.1.0"
```
If you want to install it locally, do :
```sh
git clone https://github.com/Dalejosne/Automata
```
And every files you need will be on the subdirectory Automata/game_of_life.

## Doc
After having installed it, do :
```
cargo doc
```
And then open the file index.html on the directory target/doc/game_of_life

## Examples

See the "examples" repository. To run them, do :
```sh
cargo run --example basic
```

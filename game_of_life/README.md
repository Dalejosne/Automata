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

## Quickstart

To install, see installation instructions.\
Then, try this code :
```rust
use std::{time::Duration, thread::sleep, process};
use gol_lib::*;
use crossterm;

fn main(){
	enter_alternate().unwrap();
	hide_cursor().unwrap();
	set_title("Game of Life".to_string()).unwrap();
	let mut g = GameOfLife::new(45, 25);
	g.set_element(5, 5);
	g.set_element(6, 5);
	g.set_element(7, 5);
	g.set_element(6, 6);
	clear();
	reverse();
	loop {
		g.show();
		g.update();
		refresh();
		sleep(Duration::from_millis(500));
		// To end the program if control c is pressed
		if crossterm::event::poll(Duration::from_secs(0)).unwrap() {
			if is_ctrl_c(crossterm::event::read().unwrap()).unwrap() {
				leave_alternate().unwrap();
				show_cursor().unwrap();
				process::exit(0);
			}
		}
		clear();
	}
}
```

## Install
To install it as a dependency, just add this line to your Cargo.toml :
```toml
[dependencies]
gol-lib = "0.1.3"
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

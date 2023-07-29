// Copyright Damien Lejosne 2021. See LICENSE for more informations
//! This crate allow you tu create an play the famous "game of life" of John Conway.\
//! ***But what is game of life ?***\
//! It is an automata invented by John Conway in the 70s.\
//! It is composed of an infinite grid (in practice it is finite), of which cell can be in two state : dead or alive.\
//! The neigbourhood of a cell is the 8 cells which around it (e.g bottom, top, left, right and diagonals).\
//! If an alive cell has two or three neighboors who are alive, it becomes alive. Else, it dies.\
//! If a dead cell has three neighboors who are alive, it becomes alive. Else, it stays dead.\
//! So what you have to do is just create an automata, set some cells (to meke them alive) and to see how it evoluates !\
//! ***Have fun !***
mod ansi_rust;
pub use ansi_rust::*;

///Class of game of life.\
///To create a new game of life with a grid of 25 collumns wide and 45 lines height, use :\
///```rust
///fn main(){
///    let mut game = GameOfLife::new(25, 45);
///}
///```
pub struct GameOfLife{
	nb_col:usize,
	nb_lig:usize,

	grid: Vec<Vec<bool>>,
}
impl GameOfLife{
	pub fn new(nb_col:usize, nb_lig:usize) -> GameOfLife{
		let mut new_g = GameOfLife{
							nb_col,
							nb_lig,
							grid: Vec::with_capacity(nb_lig)
						 };
		for i in 0..nb_lig{
			new_g.grid.push(Vec::with_capacity(nb_col));
			for _ in 0..nb_col {
				new_g.grid[i].push(false);
			}
		}
		new_g
	}
	///Function to set the value of an element.\
	///For exemple, to set the element at position 5, 5 (e.g. to make the cell alive), do :\
	/// /!\ Pay attention ! : It is the collumn first, and **after** the line.\
	///```rust
	///fn main(){
	///    let mut game = GameOfLife::new(25, 45);
	///    game.set_element(5,5);
	///}
	///```
	pub fn set_element(&mut self, i_col:usize, i_lig:usize){
		self.grid[i_lig][i_col] = true;
	}
	///Function to reset the value of an element.\
	///For exemple, to reset the element at position 5, 5 (e.g. to kill the cell), do :\
	/// /!\ Pay attention ! : It is the collumn first, and **after** the line.\
	///```rust
	///fn main(){
	///    let mut game = GameOfLife::new(25, 45);
	///    game.unset_element(5,5);
	///}
	///```
	pub fn unset_element(&mut self, i_col:usize, i_lig:usize){
		self.grid[i_lig][i_col] = false;
	}
	///Function to show the actual state of the game.\
	///For exemple :\
	///```rust
	///fn main(){
	///    let mut game = GameOfLife::new(25, 45);
	///    game.show();
	///}
	///```
	pub fn show(&self){
		for i in 0..self.nb_lig{
			for j in 0..self.nb_col{
				if self.grid[i][j] {
					show('#', j, i);
				}else{
					show(' ', j, i);
				}
			}
		}
	}
	///Function which update the game (e.g. pass to the next state)\
	///For exemple :\
	///```rust
	///fn main(){
	///    let mut game = GameOfLife::new(25, 45);
	///    game.update();
	///}
	///```
	pub fn update(&mut self){
		let nb_col:usize = self.nb_col;
		let nb_lig:usize = self.nb_lig;

		//Init the future value of the grid
		let mut new_grid:Vec<Vec<bool>> = Vec::with_capacity(nb_lig);
		for i in 0..nb_lig{
			new_grid.push(Vec::with_capacity(nb_col));
			for _ in 0..nb_col {
				new_grid[i].push(false);
			}
		}

		//Compute
		for ilig in 1..nb_lig-1{
			for icol in 1..nb_col-1{
				let mut nb_vies = 0;
				for i in 0..3{
					for j in 0..3{
						if !(i==1 && j==1) && self.grid[ilig+j-1][icol+i-1] {
							nb_vies+=1;
						}
					}
				}
				if nb_vies == 3 || (nb_vies == 2 && self.grid[ilig][icol]) {
					new_grid[ilig][icol] = true;
				}
			}
		}

		//Copy the future value of the grid onto the grid
		for ilig in 1..nb_lig-1{
			for icol in 1..nb_col-1{
				self.grid[ilig][icol] = new_grid[ilig][icol];
			}
		}
	}
	///Funtion to get the grid of the automata
	pub fn get_grid(&self) -> &Vec<Vec<bool>> {
		&self.grid
	}
}

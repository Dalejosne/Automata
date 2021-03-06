// copyright Damien Lejosne 2021. See LICENSE for more informations
//! This crate will be usefull to play with diplaying on the terminal

///Reverse the color of the text which will be write on the screen
pub fn reverse(){
	print!("\x1b[7m");
}
///Underline the text which will be write on the screen
pub fn underline(){
	print!("\x1b[4m");
}
///Reset every display attributes
pub fn reset(){
	print!("\x1b[0m");
}
///Print a caracter at pos x,y
pub fn show(c:char, x:usize, y:usize){
	print!("\x1B[{};{}H{}", y, x , c);
}
///Clear the screen
pub fn clear(){
	print!("\x1B[2J");
}
///Refresh the screen e.g. show it
pub fn refresh(){
	println!()
}

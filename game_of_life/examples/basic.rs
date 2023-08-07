use std::time::Duration;
use std::thread::sleep;
use gol_lib::*;
use crossterm;

fn main(){
	enter_alternate().unwrap();
	hide_cursor().unwrap();
	crossterm::terminal::enable_raw_mode().unwrap();
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
		clear();
	}
	crossterm::terminal::disable_raw_mode().unwrap();
	leave_alternate().unwrap();
}

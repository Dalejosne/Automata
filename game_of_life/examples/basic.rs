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

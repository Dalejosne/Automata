use std::time::Duration;
use std::thread::sleep;
use gol_lib::*;

fn main(){
	enter_alternate().unwrap();
	hide_cursor();
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
	leave_alternate().unwrap();
}

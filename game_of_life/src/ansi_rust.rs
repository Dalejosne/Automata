// copyright Damien Lejosne 2021. See LICENSE for more informations
//! This crate will be usefull to play with diplaying on the terminal
use crossterm::{
    self, cursor,
    event::{Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use std::io;

///Reverse the color of the text which will be write on the screen
pub fn reverse() {
    print!("\x1b[7m");
}
///Underline the text which will be write on the screen
pub fn underline() {
    print!("\x1b[4m");
}
///Reset every display attributes
pub fn reset() {
    print!("\x1b[0m");
}
///Print a caracter at pos x,y
pub fn show(c: char, x: usize, y: usize) {
    print!("\x1B[{};{}H{}", y, x, c);
}
///Clear the screen
pub fn clear() {
    execute!(
        io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}
///Refresh the screen e.g. show it
pub fn refresh() {
    println!()
}

///Enter the alternate terminal not to clear all of it
pub fn enter_alternate() -> Result<(), io::Error> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

///Leave the alternate screen and come back to the old terminal
pub fn leave_alternate() -> Result<(), io::Error> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

///To hide the cursor
pub fn hide_cursor() -> Result<(), io::Error> {
    execute!(io::stdout(), cursor::Hide)?;
    Ok(())
}

///To show the cursor
pub fn show_cursor() -> Result<(), io::Error> {
    execute!(io::stdout(), cursor::Show)?;
    Ok(())
}

///To set the title of the window
pub fn set_title(title: String) -> Result<(), io::Error> {
    execute!(io::stdout(), SetTitle(title))?;
    Ok(())
}

/// To check if an event is control-c
pub fn is_ctrl_c(event: Event) -> Result<bool, io::Error> {
    match event {
        Event::Key(keyevent) => {
            if keyevent.code == KeyCode::Char('c')
                && keyevent.kind == KeyEventKind::Press
                && keyevent.modifiers == KeyModifiers::CONTROL
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

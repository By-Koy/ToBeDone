use std::fmt::Error;
use std::error::Error as Err;

use crossterm::event::read;
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct State {
    counter: u8,
    exit: bool,
} impl State {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Err>> {

        while !self.exit {
            main(self, terminal)?;
        }
        Ok(())
    }
} 

pub fn main(app: &mut State, term: &mut DefaultTerminal) -> Result<(), Box<dyn Err>> {
    let input = read()?;
    
    term.draw(render)?;

    if input.is_key_press() {
        app.exit = true;
    }

    Ok(())
}
fn render(frame: &mut Frame) {
    frame.render_widget("hellodfgjdfhjfghj world", frame.area());
}
use std::fmt::Error;
use std::error::Error as Err;

use crossterm::event::read;
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct State {
    counter: u8,
    exit: bool,
} impl State {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn Err>> {

        term.draw(|frame| self.render(frame))?;

        while !self.exit {
            main(self)?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget("self.counter", frame.area());
    }
} 

pub fn main(app: &mut State) -> Result<(), Box<dyn Err>> {
    let input = read()?;

    if input.is_key_press() {
        app.exit = true;
    }

    Ok(())
}
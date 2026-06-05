use std::error::Error;
use crossterm::event::read;
use ratatui::{DefaultTerminal, Frame};

pub fn main(term: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let input = read().expect("input unreachable");
    
    loop {
        term.draw(render)?;

        if input.is_key_press() {
            break Ok(())
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hellodfgjdfhjfghj world", frame.area());
}
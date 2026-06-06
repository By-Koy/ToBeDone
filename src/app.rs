use std::{fs, env};
use std::error::Error;

use crossterm::event::read;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct State {
    exit: bool,
    file: Vec<String>,
} impl State {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {

        let args: Vec<String> = env::args().collect();

        let file_path = &args[1];
        self.file = fs::read_to_string(file_path)
            .expect("File couldn't be read")
            .split("\n").map(|s| s.to_string()).collect();

        term.draw(|frame| self.draw(frame))?;

        while !self.exit {
            main(self)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
impl Widget for &State {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ToBeDone ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let file_contents = Text::from(self.file.iter().map(|s| Line::from(&s[..]))
                                        .collect::<Vec<_>>());

        Paragraph::new(file_contents)
            .block(block)
            .render(area, buf);
    }
}

pub fn main(app: &mut State) -> Result<(), Box<dyn Error>> {
    let input = read()?;

    if input.is_key_press() {
        app.exit = true;
    }

    Ok(())
}
use std::error::Error;

use crate::file;

use crossterm::{ event::{self, Event, KeyCode} };
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Position},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct Cursor {
    line: u16,
    collum: u16
}

#[derive(Debug, Default)]
pub struct State {
    exit: bool,
    cursor: Cursor,
    constraints: Rect,
    pub contents: Vec<String>,
    pub id: String,
} impl State {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.constraints = term.get_frame().area();

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            main(self)?;
        }

        file::exit(self);
        println!("{self:?}");

        Ok(())
    }

    fn write(&mut self, char: char) {
        todo!();
    }

    fn backspace(&mut self) {
        todo!()
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.collum <= 0 { return }
        self.cursor.collum-=1;
    }

    // remember to add overflow
    fn move_cursor_right(&mut self) {
        if self.cursor.collum >= self.constraints.width { return }
        self.cursor.collum+=1;
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.line <= 0 { return }
        self.cursor.line-=1;
    }

    // remember to add overflow
    fn move_cursor_down(&mut self) {
        if self.cursor.line >= self.constraints.height { return }
        self.cursor.line+=1;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.set_cursor_position(Position::new(self.cursor.collum, self.cursor.line));
        frame.render_widget(self, frame.area());
    }

} impl Widget for &State {

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

        let file_contents = Text::from(self.contents.iter().map(|s| Line::from(&s[..]))
                                        .collect::<Vec<_>>());

        Paragraph::new(file_contents)
            .block(block)
            .render(area, buf);
    }
}

pub fn main(app: &mut State) -> Result<(), Box<dyn Error>> {
    let input = event::read()?;

    if input.is_key_press() {
        if let Event::Key(key_event) = input {
            match key_event.code {
                KeyCode::Left => app.move_cursor_left(),
                KeyCode::Right => app.move_cursor_right(),
                KeyCode::Down => app.move_cursor_down(),
                KeyCode::Up => app.move_cursor_up(),
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char('q') => app.exit = true,
                KeyCode::Char(char) => app.write(char),
                _ => {}
            }
        }
    }

    Ok(())
}
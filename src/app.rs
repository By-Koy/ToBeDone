use std::error::Error;

use crate::file;

use crossterm::{ event::{self, Event, KeyCode} };
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Position, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}
};

#[derive(Debug, Default)]
struct Cursor {
    line: u16,
    column: u16
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
        self.constraints.width = self.constraints.width-3;
        self.constraints.height = self.constraints.height-3;

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            main(self)?;
        }

        file::exit(self);
        println!("{self:?}");

        Ok(())
    }

    fn write(&mut self, char: char) {
        let line_index: usize = usize::from(self.cursor.line);
        let char_index: usize = usize::from(self.cursor.column);

        self.contents[line_index].insert(char_index, char);
        self.cursor.column += 1;
    }

    fn backspace(&mut self) {
        let line_index: usize = usize::from(self.cursor.line);
        let char_index: usize = usize::from(self.cursor.column);
        
        if char_index == 0 {
            match line_index {
                0 => return,
                _ => self.collapse_line(),
            };
        } else {
            self.contents[line_index].remove(char_index-1);
            self.move_cursor_left();
        }
    }

    fn collapse_line(&mut self) {
        let line_index: usize = usize::from(self.cursor.line);
        let collapsed: &str = &self.contents[line_index].clone();

        self.cursor.column = self.contents[line_index-1].len().try_into().unwrap();
        self.move_cursor_up();

        self.contents[line_index-1] += collapsed;
        self.contents.remove(line_index);

    }

    fn move_cursor(&mut self, direction: KeyCode) {
        let line_index: usize = usize::from(self.cursor.line);
        let char_index: usize = usize::from(self.cursor.column);

        let line_length: usize = usize::from(self.contents[line_index].len());

        if direction == KeyCode::Left && char_index == 0 { self.collapse_cursor_left(); return }

        if direction == KeyCode::Right && char_index == line_length { self.collapse_cursor_right(); return }

        match direction {
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            _ => return
        };

    }

    fn collapse_cursor_left(&mut self) {
        let line_index: usize = usize::from(self.cursor.line);
        let line_length: usize = usize::from(self.contents[line_index-1].len());

        self.cursor.column = line_length.try_into().unwrap();
        self.move_cursor_up();
    }

    fn collapse_cursor_right(&mut self) {
        self.cursor.column=0;
        self.move_cursor_down();
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.column <= 0 { return }
        self.cursor.column-=1;
    }

    // remember to add overflow
    fn move_cursor_right(&mut self) {
        if self.cursor.column >= self.constraints.width { return }
        self.cursor.column+=1;
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.line <= 0 { return }
        self.cursor.line-=1;
    }

    // remember to add overflow
    fn move_cursor_down(&mut self) {
        if usize::from(self.cursor.line) >= self.contents.len()-1 { return }
        self.cursor.line+=1;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.set_cursor_position(Position::new(self.cursor.column+1, self.cursor.line+1));
        frame.render_widget(self, frame.area());
    }

} impl Widget for &State {

        fn render(self, area: Rect, buf: &mut Buffer) {

        let title = Line::from(" ToBeDone ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            format!("{}, {} ", self.cursor.line, self.cursor.column).into()
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
                KeyCode::Left => app.move_cursor(KeyCode::Left),
                KeyCode::Right => app.move_cursor(KeyCode::Right),
                KeyCode::Down => app.move_cursor(KeyCode::Down),
                KeyCode::Up => app.move_cursor(KeyCode::Up),
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char('q') => app.exit = true,
                KeyCode::Char(char) => app.write(char),
                KeyCode::Enter => app.write('\n'),
                _ => {}
            }
        }
    }

    Ok(())
}
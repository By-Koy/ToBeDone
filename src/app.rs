use std::error::Error;

use crate::file;

use crossterm::{ event::{self, Event, KeyCode} };
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Position, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}
};

#[derive(Debug, Default)]
struct Cursor {
    line_vis: u16,
    column_vis: u16,
    line: usize,
    column: usize
} impl Cursor {

    //Change values directly
        pub fn add_line(&mut self, value: u16) {
            self.line_vis+=value;
            self.line+=usize::from(value);
        }

        pub fn remove_line(&mut self, value: u16) {
            self.line_vis-=value;
            self.line-=usize::from(value);
        }

        pub fn set_line(&mut self, value: u16) {
            self.line_vis=value;
            self.line=usize::from(value);
        }

        pub fn add_column(&mut self, value: u16) {
            self.column_vis+=value;
            self.column+=usize::from(value);
        }

        pub fn remove_column(&mut self, value: u16) {
            self.column_vis-=value;
            self.column-=usize::from(value);
        }

        pub fn set_column(&mut self, value: u16) {
            self.column_vis=value;
            self.column=usize::from(value);
        }

    // Move cursor automatically
        fn move_left(&mut self) {
            if self.column <= 0 { return }
            self.remove_column(1);
        }

        fn move_right(&mut self) {
            if self.column >= self.constraints.width { return }
            self.add_column(1);
        }

        fn move_up(&mut self) {
            if self.line <= 0 { return }

            let len: usize = self.contents[usize::from(self.line-1)].len().try_into().unwrap();

            self.remove_line(1);

            if self.column > len {
                self.column = len;
            }
        }

        fn move_down(&mut self) {
            if self.line >= self.contents.len()-1 { return }

            let len: usize = self.contents[usize::from(self.line+1)].len().try_into().unwrap();

            self.add_line(1);

            if self.column > len {
                self.column = len;
            }

            if self.line >= self.constraints.height {
                self.viewport+=1;
            }
        }

    // Collapse cursor
        fn collapse_left(&mut self) {
            if self.line == 0 { return }

            let line_length: usize = usize::from(self.contents[usize::from(self.line-1)].len());

            self.column = line_length.try_into().unwrap();
            self.move_up();
        }

        fn collapse_right(&mut self) {
            if usize::from(self.line) >= self.contents.len()-1 { return }

            self.set_column(0);
            self.move_down();
        }
}



#[derive(Debug, Default)]
pub struct State {
    exit: bool,
    cursor: Cursor,
    constraints: Rect,
    viewport: usize,
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
        self.contents[self.cursor.line].insert(self.cursor.column, char);
        self.cursor.move_right();
    }

    fn backspace(&mut self) {
        if self.cursor.column == 0 {
            match self.cursor.line {
                0 => return,
                _ => self.collapse_line(),
            };
        } else {
            self.contents[self.cursor.line].remove(self.cursor.column-1);
            self.cursor.move_left();
        }
    }

    fn new_line(&mut self) {
        let remainder: String = self.contents[self.cursor.line].chars().skip(usize::from(self.cursor.column)).collect();

        self.contents.insert(self.cursor.line+1, remainder);
        self.cursor.move_down();
        self.cursor.set_column(0);

    }

    fn move_cursor(&mut self, direction: KeyCode) {
        let line_length: usize = usize::from(self.contents[self.cursor.line].len());

        if direction == KeyCode::Left && self.cursor.column == 0 { self.cursor.collapse_left(); return }

        if direction == KeyCode::Right && self.cursor.column == line_length { self.cursor.collapse_right(); return }

        match direction {
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self.cursor.move_right(),
            KeyCode::Down => self.cursor.move_down(),
            KeyCode::Up => self.cursor.move_up(),
            _ => return
        };

    }

    fn collapse_line(&mut self) {
        let collapsed: &str = &self.contents[self.cursor.line].clone();

        self.cursor.set_column(self.contents[self.cursor.line-1].len().try_into().unwrap());
        self.cursor.move_up();

        self.contents[self.cursor.line-1] += collapsed;
        self.contents.remove(self.cursor.line);

    }

    fn draw(&self, frame: &mut Frame) {
        frame.set_cursor_position(Position::new(self.cursor.column_vis+1, self.cursor.line_vis+1));
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

        let file_contents = Text::from(self.contents.iter().skip(self.viewport).map(|s| Line::from(&s[..]))
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
                KeyCode::Enter => app.new_line(),
                _ => {}
            }
        }
    }

    Ok(())
}
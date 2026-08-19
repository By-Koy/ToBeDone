use std::error::Error;

use crate::file;
use crate::Args;

use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::{ event::{self, Event, KeyCode} };
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::layout::Layout;
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer,
    layout::{Position, Rect}, 
    style::{Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget}
};

#[derive(Debug, Default)]
struct Cursor {
    line_vis: u16,
    column_vis: u16,
    line: usize,
    column: usize,

    area: Option<Rect>
} impl Cursor {

    // Return constraints
        pub fn constraints(&self) -> Rect {
        if let Some(rect) = self.area {
                Rect::new(rect.x, rect.y, rect.width-3, rect.height-3)
            } else {
                Rect::new(0, 0, 0, 0)
            }
        }

    // Change values directly
        pub fn add_line(&mut self, value: u16) {
            self.line+=usize::from(value);

            if self.line_vis == self.constraints().height { return }
            
            self.line_vis+=value;
        }

        pub fn lower_line(&mut self, value: u16) {
            if self.line <= 0 { return }

            self.line-=usize::from(value);

            if self.line_vis == 0 { return }

            self.line_vis-=value;
        }

        // pub fn set_line(&mut self, value: u16) {
        //     self.line_vis=value;
        //     self.line=usize::from(value);
        // }

        pub fn add_column(&mut self, value: u16) {
            self.column+=usize::from(value);

            if self.column_vis == self.constraints().width { return }

            self.column_vis+=value;
        }

        pub fn lower_column(&mut self, value: u16) {
            self.column-=usize::from(value);

            if self.column_vis == 0 { return }

            self.column_vis-=value;
        }

        pub fn set_column(&mut self, value: u16) {
            self.column_vis=value;
            self.column=usize::from(value);
        }

    // Move cursor automatically
        fn move_left(&mut self) {
            if self.column <= 0 { return }
            self.lower_column(1);
        }

        fn move_right(&mut self) {
            self.add_column(1);
        }

        fn move_up(&mut self, len: u16) {
            if self.line <= 0 { return }

            self.lower_line(1);

            if self.column > usize::from(len) {
                self.set_column(len);
            }
        }

        fn move_down(&mut self, len: u16, len_max: usize) {
            if self.line >= len_max { return }

            self.add_line(1);

            if self.column > usize::from(len) {
                self.set_column(len);
            }
        }

    // Collapse cursor
        fn collapse_left(&mut self, len: u16) {
            if self.line == 0 { return }

            self.set_column(len);
            self.lower_line(1);
        }

        fn collapse_right(&mut self, len_max: usize) {
            if usize::from(self.line) >= len_max { return }

            self.set_column(0);
            self.add_line(1);
        }
}

#[derive(Debug, Default)]
struct FormatDisplay {
    hidden: bool
} impl Widget for &FormatDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let split = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);

        Block::new()
                .title_bottom("Options".blue().bold())
                .title_alignment(Alignment::Center)
                .render(area, buf);

        let instructions_left = Text::from(vec![
                                    "App controls (hold ctrl!)".blue().bold().into(),
                                    "<q>".blue().bold().into()
                        ]);

        Paragraph::new(instructions_left.clone()).alignment(Center).render(split[0], buf);
        Paragraph::new(instructions_left).alignment(Center).render(split[1], buf);
    }
}

#[derive(Debug, Default)]
pub struct State {
    exit: bool,
    format_display: FormatDisplay,
    cursor: Cursor,
    args: Args,
    pub contents: Vec<String>,
    pub id: String,
} impl State {
    pub fn run(&mut self, term: &mut DefaultTerminal, args: Args) -> Result<(), Box<dyn Error>> {
        self.args = args;

        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            main(self)?;
        }

        file::exit(self);

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.set_cursor_position(Position::new(self.cursor.column_vis+1, self.cursor.line_vis+1));

        if self.format_display.hidden {
            let display = Layout::vertical( [Constraint::Min(6), Constraint::Length(5)]).split(frame.area());
            self.cursor.area = Some(display[0]);

            frame.render_widget(&self.format_display, display[1]);
            frame.render_widget(&*self, display[0]);
        } else {
            self.cursor.area = Some(frame.area());
            frame.render_widget(&*self, frame.area());
        }
    }


    fn write(&mut self, char: char) {
        self.contents[self.cursor.line].insert(self.cursor.column, char);
        self.move_cursor(KeyCode::Right);
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

        self.contents[self.cursor.line].replace_range(self.cursor.column.., "");
        self.contents.insert(self.cursor.line+1, remainder);

        self.move_cursor(KeyCode::Down);
        self.cursor.set_column(0);

    }

    fn collapse_line(&mut self) {
        let collapsed: &str = &self.contents[self.cursor.line].clone();

        self.move_cursor(KeyCode::Left);

        self.contents[self.cursor.line] += collapsed;
        self.contents.remove(self.cursor.line+1);

    }

    fn move_cursor(&mut self, direction: KeyCode) {
        let line_len: usize = usize::from(self.contents[self.cursor.line].len());
        let file_len: usize = self.contents.len()-1;

        let len: u16 = if self.cursor.line == 0 || self.cursor.line == file_len { 0
                } else if direction == KeyCode::Down {
                    self.contents[self.cursor.line+1].len().try_into().unwrap()
                } else {
                    self.contents[usize::from(self.cursor.line-1)].len().try_into().unwrap()
                };

        if direction == KeyCode::Left && self.cursor.column == 0 { self.cursor.collapse_left(len); return }

        if direction == KeyCode::Right && self.cursor.column == line_len { self.cursor.collapse_right(file_len); return }

        match direction {
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self.cursor.move_right(),
            KeyCode::Down => self.cursor.move_down(len, file_len),
            KeyCode::Up => self.cursor.move_up(len),
            _ => return
        };

    }

    fn toggle_format_display(&mut self) {
        if self.format_display.hidden { self.format_display.hidden = false }
            else { self.format_display.hidden = true }
    }

    fn parse_mod(&mut self, event: KeyEvent) {
        if event.modifiers == KeyModifiers::CONTROL || event.modifiers == KeyModifiers::SUPER {
            match event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('a') => self.toggle_format_display(),
                _ => return
            }
        }

    }

} impl Widget for &State {

        fn render(self, area: Rect, buf: &mut Buffer) {

        let title = Line::from(" ToBeDone ".bold());

        let file_contents: Text = self.contents.iter().skip(self.cursor.line-usize::from(self.cursor.line_vis))
                            .map(|s| Line::from(&s[self.cursor.column-usize::from(self.cursor.column_vis)..])).collect();

        // .map(|l: Line| -> Line { match &l.to_string()[0..l.to_string().char_indices().nth(1).map(|(n, _)| {return n;}).unwrap()] {
        //     "#b" => l.style(Color::Black),
        //     "#B" => l.style(Color::LightBlue),
        //     "#R" => l.style(Color::Red),
        //     "#Y" => l.style(Color::LightYellow),
        //     "#M" => l.style(Color::Magenta),
        //     "#C" => l.style(Color::Cyan),
        //     "#G" => l.style(Color::Green),
        //     "**" => l.bold(),
        //     "* " => l.italic(),
        //     _ => l
        // }})

        let instructions = if self.args.debug {
                                Line::from( vec![
                                            " Quit ".into(),
                                            "<Q> ".blue().bold(),
                                            format!("l:{}-v:{}, c:{}-v:{} - ch:{}, cw:{} , fd:{}",
                                                self.cursor.line, self.cursor.line_vis,
                                                self.cursor.column, self.cursor.column_vis,
                                                self.cursor.constraints().height,
                                                self.cursor.constraints().width,
                                                self.format_display.hidden).into() ])
                            } else {
                                Line::from(vec![
                                            " Quit ".into(),
                                            "<Q> ".blue().bold() ])
                            };

        let block = Block::bordered()
                                .title(title.centered())
                                .title_bottom(instructions.centered())
                                .border_set(border::THICK);

        Paragraph::new(file_contents)
            .block(block)
            .render(area, buf);
    }
}

pub fn main(app: &mut State) -> Result<(), Box<dyn Error>> {
    let input = event::read()?;

    if input.is_key_press() {
        if let Event::Key(key_event) = input {
            match key_event.modifiers {
                KeyModifiers::NONE => match key_event.code {
                        KeyCode::Left => app.move_cursor(KeyCode::Left),
                        KeyCode::Right => app.move_cursor(KeyCode::Right),
                        KeyCode::Down => app.move_cursor(KeyCode::Down),
                        KeyCode::Up => app.move_cursor(KeyCode::Up),
                        KeyCode::Backspace => app.backspace(),
                        KeyCode::Char(char) => app.write(char),
                        KeyCode::Enter => app.new_line(),
                        _ => {}
                    },
                _ => app.parse_mod(key_event),
            }
        }
    }

    Ok(())
}
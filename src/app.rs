use std::error::Error;

use crate::Args;
use crate::file;
use crate::ARGS;

use crossterm::
    event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers
};
use ratatui::layout::Margin;
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer,
    layout::{Position, Rect, Layout, Alignment, Constraint},
    layout::HorizontalAlignment::Center,
    style::{Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    prelude::{Span, Style, Color}
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

        // Divide the area
        let usable_area = area.inner(Margin::new(1, 1));

        let block_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);
        
        let usable_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(usable_area);

        // Display app controls
        let instructions_app = Text::from(vec![
                                    Line::from(vec![Span::from(" <A> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- Open this menu".into()
                                    ]),
                                    Line::from(vec![Span::from(" <Q> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- Quit the program".into()
                                    ])
                        ]);

        Block::new()
            .title("App controls (use with CTRL)".blue().bold())
            .title_alignment(Alignment::Center)
            .render(block_layout[0], buf);

        Paragraph::new(instructions_app).alignment(Center).render(usable_layout[0], buf);

        // Display fromating controls
        Block::new()
            .title("Formating (use with SUPER)".blue().bold())
            .title_alignment(Alignment::Center)
            .render(block_layout[1], buf);

        // (subdivide the right side if needed)
        let right = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(usable_layout[1]);

        let instructions_format = Text::from(vec![
                                    Line::from(vec![Span::from(" <B> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- Make a line BOLD".into()
                                    ]),
                                    Line::from(vec![Span::from(" <I> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- ITALICISE a line".into()
                                    ]),
                                    Line::from(vec![Span::from(" <A> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- add a BACKGROUND".into()
                                    ]),
                                    Line::from(vec![Span::from(" <S> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- STRIKETHROUGH the line".into()
                                    ]),
                                    Line::from(vec![Span::from(" <U> ").style(Style::new().fg(Color::Blue).bold()),
                                                            "- give it an UNDERLINE".into()
                                    ]),
                        ]);

        if &instructions_format.lines.len() > &usize::from(usable_area.height) {
            Paragraph::new(instructions_format.clone())
                            .alignment(Center)
                            .render(right[0], buf);

            Paragraph::new(
                    instructions_format.lines.into_iter()
                    .skip(usize::from(usable_area.height)).collect::<Vec<Line>>())
                        .alignment(Center)
                        .render(right[1], buf);
        } else {
            Paragraph::new(instructions_format.clone()).alignment(Center).render(usable_layout[1], buf);
        }

        let title  = if ARGS.lock().unwrap().debug {
                    Line::from(vec![
                        "Options ".blue().bold(),
                        format!("area: x:{}, y:{}, usable: x:{}, y:{}",
                            area.width, area.height,
                            usable_area.width, usable_area.height).into()
                    ])
        } else {
            Line::from("Options ".blue().bold())
        };

        Block::new()
                .title_bottom(title)
                .title_alignment(Alignment::Center)
                .render(area, buf);
    }
}

#[derive(Debug, Default)]
pub struct State<'a> {
    exit: bool,
    format_display: FormatDisplay,
    cursor: Cursor,
    args: Args,
    pub contents: Text<'a>,
    pub id: String,
} impl<'a> State<'a> {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.args = ARGS.lock().unwrap().clone();
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
            let display = Layout::vertical( [Constraint::Min(8), Constraint::Max(10)]).split(frame.area());
            self.cursor.area = Some(display[0]);

            frame.render_widget(&self.format_display, display[1]);
            frame.render_widget(&*self, display[0]);
        } else {
            self.cursor.area = Some(frame.area());
            frame.render_widget(&*self, frame.area());
        }
    }


    fn write(&mut self, char: char) {
        self.contents.lines[self.cursor.line].spans.insert(self.cursor.column, Span::raw(String::from(char)));
        self.move_cursor(KeyCode::Right);
    }

    fn backspace(&mut self) {
        if self.cursor.column == 0 {
            match self.cursor.line {
                0 => return,
                _ => self.collapse_line(),
            };
        } else {
            self.contents.lines[self.cursor.line].spans.remove(self.cursor.column-1);
            self.cursor.move_left();
        }
    }

    fn new_line(&mut self) {
        let remainder: Vec<Span> = self.contents.lines[self.cursor.line].clone().into_iter().skip(usize::from(self.cursor.column)).collect();

        self.contents.lines[self.cursor.line].spans.drain(self.cursor.column..).collect::<Vec<Span>>();
        self.contents.lines.insert(self.cursor.line+1, Line::from(remainder));

        self.move_cursor(KeyCode::Down);
        self.cursor.set_column(0);

    }

    fn collapse_line(&mut self) {
        let mut collapsed: Vec<Span> = self.contents.lines[self.cursor.line].to_string().chars().map(|c| Span::raw(String::from(c))).collect();

        self.move_cursor(KeyCode::Left);

        self.contents.lines[self.cursor.line].spans.append(&mut collapsed);
        self.contents.lines.remove(self.cursor.line+1);

    }

    fn move_cursor(&mut self, direction: KeyCode) {
        let line_len: usize = usize::from(self.contents.lines[self.cursor.line].iter().len());
        let file_len: usize = self.contents.lines.len()-1;

        let len: u16 = if self.cursor.line == 0 || self.cursor.line == file_len { 0
                } else if direction == KeyCode::Down {
                    self.contents.lines[self.cursor.line+1].iter().len().try_into().unwrap()
                } else {
                    self.contents.lines[usize::from(self.cursor.line-1)].iter().len().try_into().unwrap()
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
        if event.modifiers == KeyModifiers::CONTROL {
            match event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('a') => self.toggle_format_display(),
                _ => return
            };

        } else if event.modifiers == KeyModifiers::ALT {
            match event.code {
                KeyCode::Char('b') => self.bold(),
                _ => return
            };

        }

    }

    fn bold(&mut self) {

    }

} impl<'a> Widget for &State<'a> {

        fn render(self, area: Rect, buf: &mut Buffer) {

        let title = Line::from(" ToBeDone ".bold());

        let instructions = if self.args.debug {
                                Line::from( vec![
                                            " Menu ".into(),
                                            "<ctrl+A> ".blue().bold(),
                                            format!(" l:{}-v:{}, c:{}-v:{} - ch:{}, cw:{} , fd:{} ",
                                                self.cursor.line, self.cursor.line_vis,
                                                self.cursor.column, self.cursor.column_vis,
                                                self.cursor.constraints().height,
                                                self.cursor.constraints().width,
                                                self.format_display.hidden).into() ])
                            } else {
                                Line::from(vec![
                                            " Menu ".into(),
                                            "<A> ".blue().bold() ])
                            };

        let block = Block::bordered()
                                .title(title.centered())
                                .title_bottom(instructions.centered())
                                .border_set(border::THICK);

        Paragraph::new(self.contents.clone())
            .scroll((self.cursor.line as u16 - self.cursor.line_vis, self.cursor.column as u16 - self.cursor.column_vis))
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
use std::io::stdout;
use std::error::Error;

use ratatui::prelude::Terminal;
use ratatui::backend::CrosstermBackend;
use crossterm::terminal::*;
use crossterm::ExecutableCommand;

fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;
    

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

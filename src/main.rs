use std::error::Error;

mod app;


fn main() -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| app::State::default().run(terminal))?;

    Ok(())
}

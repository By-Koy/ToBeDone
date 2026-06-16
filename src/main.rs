use std::env;

mod app;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = app::State::default();

    file::main(&mut app, args);
    ratatui::run(|terminal| app.run(terminal)).unwrap();
}

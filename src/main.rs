use std::env;
use std::sync::{LazyLock, Mutex};

mod app;
mod file;

#[derive(Debug, Default, Clone)]
pub struct Args {
    debug: bool,
    sample: bool,
    no_eggs: bool,
    other: Vec<String>
} impl Args {

    fn init(&mut self, input: Vec<String>) -> Vec<String> {
        let args: Vec<String> = input.clone().into_iter().filter(|s| s.starts_with("--")).collect();
        for arg in args {
            match &*arg {
                "--debug" => self.debug=true,
                "--sample" => self.sample=true,
                "--no-eggs" => self.no_eggs=true,
                _ => self.other.push(arg)
            };
        }

        if self.other.len() > 0 { println!("Unknown argument(s) {:?}.
                                            \nPossible arguments are:
                                            \n --debug - show debuging vars,
                                            \n --sample - make sample.md with provided sample text,
                                            \n --no-eggs - no eggs\n", self.other)}

        input.into_iter().filter(|s| !s.starts_with("--")).collect()
    }
}

static ARGS: LazyLock<Mutex<Args>> = LazyLock::new(|| Mutex::new(Args::default()));

fn main() {
    let input: Vec<String>  = ARGS.lock().unwrap()
                                .init(env::args().collect());

    let mut app = app::State::default();

    file::main(&mut app, input);
    ratatui::run(|terminal| app.run(terminal)).unwrap();
}

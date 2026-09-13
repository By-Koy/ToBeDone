use std::fs;
use std::path::Path;
use std::error::Error;
#[cfg(target_family = "unix")]
    use std::os::unix::fs as unix;
#[cfg(target_family = "windows")]
    use std::os::windows::fs as windows;

use ratatui::prelude::{
            text::{Text, Line, Span},
            Stylize };

use crate::app::State;
use crate::ARGS as args;

#[cfg(target_family = "unix")]
static PATH: &str  = concat!(env!("XDG_DATA_HOME"), "/TBD");
#[cfg(target_family = "windows")]
static PATH: &str = concat!(env!("LOCALAPPDATA")+"/TBD");

pub fn main(app: &mut State, input: Vec<String>) {
    if args.lock().unwrap().sample {sample(app); return}
    else if args.lock().unwrap().reset {let _ = reset(); return}

    let id: &str = if input.len() <= 1 {
                        "Recent"
                    } else {
                        &input[1]
                    };

    let check = check_path(app, &id);
    if let Err(_) = check {
        fs::write(Path::new(&format!("{PATH}/{id}.md")), format!("**{id}**"))
            .expect("unable to create/write to file, please check permissions.");

        app.contents = Text::from(Line::from(id.chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()).bold());
        app.id = id.to_string();
    }

}

fn check_path(app: &mut State, id: &str) -> Result<(), Box<dyn Error>> {

    app.contents = fs::read_to_string(Path::new(&format!("{PATH}/{id}.md")))?
                .split("\n").map(|s| s.to_string()).collect();
    app.id = String::from(id);

    Ok(())
}

fn sample(app: &mut State) {
    let sample_text: Text = Text::from(vec!(
                        Line::from(" \"It is this eternal dance,".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" that separates human beings,".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" from demons, from angels,".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" from gods.".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" And I must not forget,".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" We must not forget,".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from(" That we are human-beings.\"".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()),
                        Line::from("     --Ren Gill".to_string().chars().map(|c| Span::raw(c.to_string())).collect::<Vec<Span>>()) ));

    app.contents = sample_text;
    app.id = "Sample".to_string();
}

pub fn exit(app: &State) {
    if app.contents.lines.is_empty()  {
        fs::remove_file(Path::new(&format!("{PATH}/{}.md", &app.id)))
            .expect("unable to remove note, please check permissions");

    } else if app.id != "Recent" {
        let _ = fs::remove_file(Path::new(&format!("{PATH}/Recent.md")));

        #[cfg(target_family = "unix")]
            unix::symlink(Path::new(&format!("{PATH}/{}.md", &app.id)), Path::new(&format!("{PATH}/Recent.md")))
                .expect("unable to create symlink, please check permissions");
        #[cfg(target_family = "windows")]
            windows::symlink_file(Path::new(&format!("{PATH}/{}.md", &app.id)), Path::new(&format!("{PATH}/Recent.md")))
                .expect("unable to create symlink, please check permissions");
    }
    
    fs::write(Path::new(&format!("{PATH}/{}.md", &app.id)), app.contents.iter().map(|l| l.to_string()+"\n").collect::<Vec<String>>()
                                                                        .into_iter().collect::<String>())
            .expect("unable to create/write to file, please check permissions.");
}

pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    println!("Are you sure that you would like to remove notes? (Y/N)");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let reset: bool = match &*input {
        "Y\n" |
        "y\n" |
      "yes\n" |
      "Yes\n" => true,
        "N\n" |
        "n\n" |
       "no\n" |
       "No\n" => false,
            _ => false
    };

    if reset {

        let files = fs::read_dir(Path::new(&PATH)).expect("Folder seems empty");
        for file in files {
            fs::remove_file(file.unwrap().path())?;
        }
    }

    std::process::exit(0);
}
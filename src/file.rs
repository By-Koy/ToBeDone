use std::fs;
use std::error::Error;
use std::os::unix::fs as unix;

use crate::app::State;
use crate::Args;

pub fn main(app: &mut State, input: Vec<String>, args: &Args) {
    if args.sample {sample(app); return}

    let id: &str = if input.len() <= 1 {
                        "Recent"
                    } else {
                        &input[1]
                    };

    let check = check_path(app, &id);
    if let Err(_) = check {
        fs::write(format!("/var/local/TBD/{id}.md"), format!("**{id}**"))
            .expect("unable to create/write to file, please check permissions.");

        app.contents = vec!(format!("**{id}**"));
        app.id = id.to_string();
    }

}

fn check_path(app: &mut State, id: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("/var/local/TBD/{id}.md");

    app.contents = fs::read_to_string(path)?
                .split("\n").map(|s| s.to_string()).collect();
    app.id = String::from(id);

    Ok(())
}

fn sample(app: &mut State) {
    let sample_text: Vec<String> = vec!(" \"It is this eternal dance,".to_string(),
                                        " that separates human beings,".to_string(),
                                        " from demons, from angels,".to_string(),
                                        " from gods.".to_string(),
                                        " And I must not forget,".to_string(),
                                        " We must not forget,".to_string(),
                                        " That we are human-beings.".to_string(),
                                        "     --Ren Gill\"".to_string() );

    app.contents = sample_text;
    app.id = "Sample".to_string();
}

pub fn exit(app: &State) {
    let path = format!("/var/local/TBD/{}.md", &app.id);
    if app.contents.is_empty()  {
        fs::remove_file(&path)
            .expect("unable to remove note, please check permissions");
    } else if app.id != "Recent" {
        fs::remove_file("/var/local/TBD/Recent.md")
            .expect("unable to remove old symlink, please check permissions");

        unix::symlink(&path, "/var/local/TBD/Recent.md")
            .expect("unable to create symlink, please check permissions");
    }

    let write: String = app.contents.clone().into_iter().map(|s| format!("{}\n", &s[..])).collect();
    fs::write(&path, write)
        .expect("unable to create/write to file, please check permissions.");
}
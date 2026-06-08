use std::fs;
use std::error::Error;
use std::os::unix::fs as unix;

use crate::app::State;

pub fn main(app: &mut State, args: Vec<String>) {
    let id: &str;

    if args.len() <= 1 {
        id = "Recent";
    } else {
        id = &args[1];
    }

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
    
    if let Err(e) = fs::File::open(&path) {
        return Err(Box::new(e));
    }

    app.contents = fs::read_to_string(path)?
                .split("\n").map(|s| s.to_string()).collect();
    app.id = String::from(id);

    Ok(())
}

pub fn exit(app: &mut State) {
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
}
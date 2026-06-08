use std::fs;
use std::error::Error;

use crate::app::State;

// step 1. figure out note path from name DONE
// step 1.b. if note doesn't exist, create it
// step 1.c. if not given note name get last used
// step 2. save path DONE
// step 3. read and split file contents DONE

// let contents = fs::read_to_string("/var/local/TBD/0*")?
//                 .split("\n").map(|s| s.to_string()).collect();

pub fn main(app: &mut State, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let id: &str;

    if args.len() <= 1 {
        id = "Recent";
    } else {
        id = &args[1];
    }

    check_path(app, format!("/var/local/TBD/{id}.md"))?;
    Ok(())
}

fn check_path(app: &mut State, path: String) -> Result<(), Box<dyn Error>> {
    if let Err(e) = fs::File::open(&path) {
        return Err(Box::new(e));
    }

    app.contents = fs::read_to_string(&path)?
                .split("\n").map(|s| s.to_string()).collect();
    app.file_path = path;

    Ok(())
}
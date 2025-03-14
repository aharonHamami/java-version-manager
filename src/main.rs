mod args_utils;
use std::{env, io};

// use args_utils::UserArgs;

mod java_utils;

fn display_help() {
    println!("
    Show and manage java versions more easily.

    Usage:
    javaVM [command] [...arguments]

    Available commands:
        - root: print root folder where all jdk versions are stored.
        - list: show a list of all jdk versions.
        - use <version>: change active jdk version.
        - help: display this help page. 
    ")
}

fn main() -> io::Result<()> {
    // let args: UserArgs = args_utils::get_user_arguments();
    let args: Vec<String> = env::args().collect();
    let command: Option<String> = args.get(1).cloned();

    match command.as_deref() {
        Some("root") => {
            let java_root_path = java_utils::get_java_root_path();
            let java_root_str = java_root_path.to_str().unwrap_or("<Unrecognizable Path>");
            println!("Java root path: {}", java_root_str);
        },
        Some("list") => java_utils::list_java_versions()?,
        Some("use") => {
            let version = args.get(2).expect("Error: use: no target version specified");
            match java_utils::change_java_version(version) {
                Ok(_) => print!("Java version changed to {version}!"),
                Err(err) => print!("Couldn't change java version.\n{err}")
            }
        }
        Some("help") | None => {display_help();}
        Some(command) => {eprintln!("Unknown command \"{command}\"")}
    }

    Ok(())
}

use std::{env, fs, io, path::{Path, PathBuf}};

static JAVA_HOME: &str = "JAVA_HOME";

fn get_java_home_dir() -> String {
    let java_home_dir = env::var(JAVA_HOME).unwrap_or_else(|err| {
        panic!("Couldn't get environment variable {JAVA_HOME}.\n{err}");
    });

    java_home_dir
}

pub fn get_java_root_path() -> PathBuf {
    let home_dir = get_java_home_dir();

    let root_path = match Path::new(home_dir.as_str()).parent() {
        Some(path) => path,
        None => panic!("Couldn't open the root path of ${JAVA_HOME}. make sure you configured this path correctly")
    };

    PathBuf::from(root_path)
}

pub fn get_java_versions(path: &Path) -> Result<Vec<String>, io::Error> {
    let mut java_versions = vec![];

    let java_folders = fs::read_dir(path)?;

    for dir in java_folders {
        let dir = dir?;
        if let Some(file_name) = dir.file_name().to_str() {
            java_versions.push(String::from(file_name));
        }
    }

    Ok(java_versions)
}

pub fn list_java_versions() -> Result<(), io::Error> {
    let home_dir = get_java_home_dir();
    let root_path = get_java_root_path();

    let current_version = Path::new(&home_dir).file_name().map(|f| f.to_str());

    get_java_versions(&root_path)?.iter().for_each(|version| {
        if current_version == Some(Some(version.as_str())) {
            println!("* {version}");
        }else {
            println!("  {version}");
        }
    });

    Ok(())
}

pub fn change_java_version(target_version: &str) -> Result<(), io::Error> {
    use std::process::Command;

    let root_path = get_java_root_path();

    let java_versions = get_java_versions(&root_path)?;

    if java_versions.iter().any(|s| s == target_version) {
        let new_java_home = root_path.join(target_version);
        if let Some(new_java_home_str) = new_java_home.to_str() {
            Command::new("cmd")
                .args(&["/C", "setx", JAVA_HOME, format!("\"{new_java_home_str}\"").as_str()])
                .status()?;
        }
    }

    Ok(())
}

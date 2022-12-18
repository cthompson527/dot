use std::env;
use std::process::Command;

pub fn gclone(url: String) {
    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => "/".to_string(),
    };
    let path = format!("{}/dotfiles", home);
    Command::new("git").arg("clone").arg(url).arg(path).output().unwrap();
}

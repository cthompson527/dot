use std::env;
use std::process::Command;

fn dotfiles_dir() -> String {
    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => "/".to_string(),
    };
    format!("{}/dotfiles", home)
}

fn build_clone(git_cmd: &mut Command, url: String) {
    let path = dotfiles_dir();
    git_cmd.arg("clone").arg(url).arg(path);
}

pub fn gclone(url: String) {
    let mut git_cmd = Command::new("git");
    build_clone(&mut git_cmd, url);
    git_cmd.output().unwrap();
}

pub fn gpush(branch: String, message: String) {
    let path = dotfiles_dir();
    Command::new("git").arg("add").arg("-A").output().unwrap();
    Command::new("git").arg("commit").arg("-m").arg(message).output().unwrap();
    Command::new("git").arg("push").arg("origin").arg(format!("main:{}", branch)).output().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gclone() {
        let dot_dir = dotfiles_dir();
        let mut git_cmd = Command::new("git");
        build_clone(&mut git_cmd, "url".to_string());
        assert_eq!(git_cmd.get_program(), "git");
        let expected_args = vec!["clone", "url", &dot_dir];
        let args = git_cmd.get_args();
        for (i, arg) in args.enumerate() {
            assert_eq!(arg, expected_args[i]);
        }
    }
}

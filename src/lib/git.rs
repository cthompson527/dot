use std::env;
use std::process::Command;

#[macro_export]
macro_rules! git {
    ( $( $arg:expr ), * ) => {
        {
            let mut cmd = Command::new("git");
            $(
                cmd.arg($arg);
            )*
            &cmd.output().unwrap().stdout
        }
    };
    ( dir = $dir:expr ; $( $arg:expr ), * ) => {
        {
            let mut cmd = Command::new("git");
            cmd.current_dir($dir);
            $(
                cmd.arg($arg);
            )*
            &cmd.output().unwrap().stdout
        }
    };
}

pub fn dotfiles_dir() -> String {
    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };
    format!("{}/dotfiles", home)
}

pub fn gclone(url: String) {
    let _ = git!("clone", url, dotfiles_dir());
}

pub fn gpush(branch: String, message: String) {
    let cdir = dotfiles_dir();
    let _ = git!(dir = &cdir; "add", "-A");
    let _ = git!(dir = &cdir; "commit", "-m", message);
    let _ = git!(dir = cdir; "push", "origin", format!("main:{}", branch));
}

pub fn gstatus() {
    let cdir = dotfiles_dir();
    print!(
        "{}",
        std::str::from_utf8(git!(dir = &cdir; "status")).unwrap()
    );
}

pub fn gpull(branch: String) {
    let cdir = dotfiles_dir();
    let _ = git!(dir = cdir; "pull", "origin", format!("{}:main", branch));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotfiles_dir_when_home_env_var_is_set() {
        env::set_var("HOME", "/home/apollo");
        assert_eq!(dotfiles_dir(), "/home/apollo/dotfiles");
    }

    #[test]
    fn test_dotfiles_dir_when_home_env_var_is_not_set() {
        env::remove_var("HOME");
        assert_eq!(dotfiles_dir(), "/dotfiles");
    }
}

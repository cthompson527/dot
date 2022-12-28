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
            cmd.output().unwrap();
        }
    };
    ( dir = $dir:expr ; $( $arg:expr ), * ) => {
        {
            let mut cmd = Command::new("git");
            cmd.current_dir($dir);
            $(
                cmd.arg($arg);
            )*
            cmd.output().unwrap();
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
    git!("clone", url, dotfiles_dir());
}

pub fn gpush(branch: String, message: String) {
    let cdir = dotfiles_dir();
    git!(dir = &cdir; "add", "-A");
    git!(dir = &cdir; "commit", "-m", message);
    git!(dir = cdir; "push", "origin", format!("main:{}", branch));
}

pub fn gpull(branch: String) {
    let cdir = dotfiles_dir();
    git!(dir = cdir; "pull", "origin", format!("{}:main", branch));
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

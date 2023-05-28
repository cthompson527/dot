use crate::git::dotfiles_dir;
use std::os::unix::fs as unixfs;
use std::{env, fs, path::PathBuf, str::FromStr};
use walkdir::WalkDir;

fn walk_repo(start: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(&start)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|p| p.clone().into_path().is_file())
        .map(|e| e.into_path())
        .map(|e| e.strip_prefix(start.as_path()).unwrap().to_path_buf())
        .collect()
}

fn get_home() -> String {
    if let Ok(home_var) = env::var("HOME") {
        home_var
    } else {
        "/".to_string()
    }
}

fn resolve_path(path: PathBuf) -> Option<PathBuf> {
    if let Ok(file) = path.strip_prefix("HOME") {
        Some(PathBuf::from_str(&get_home()).unwrap().join(file))
    } else {
        match path.strip_prefix("ROOT") {
            Ok(f) => Some(PathBuf::from_str("/").unwrap().join(f)),
            Err(_) => None,
        }
    }
}

fn create_symlinks(paths: Vec<PathBuf>) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};

    for path in paths {
        if let Some(sym_path) = resolve_path(path.to_path_buf()) {
            let path = PathBuf::from_str(&dotfiles_dir()).unwrap().join(path);

            if sym_path.is_symlink() {
                let is_correct_path = match sym_path.read_link() {
                    Ok(p) => {
                        println!("Correct link: {:?}", path);
                        println!("Linked to: {:?}", p);
                        p == path
                    }
                    Err(_) => false,
                };
                if !is_correct_path {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("{} linked to wrong file", sym_path.clone().display()),
                    ));
                } else {
                    continue;
                }
            }
            if sym_path.exists() {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("{} already exists", sym_path.clone().display()),
                ));
            } else if let Some(sym_dir) = sym_path.parent() {
                fs::create_dir_all(sym_dir)?;
            }
            println!("Making link {} -> {}", sym_path.display(), path.display());

            unixfs::symlink(path.as_path(), sym_path)?;
        }
    }
    Ok(())
}

pub fn setup() -> std::io::Result<()> {
    let dot = dotfiles_dir();
    let files = walk_repo(PathBuf::from_str(&dot).unwrap());
    create_symlinks(files)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::thread;
    use std::time::Duration;
    use std::{
        env,
        io::Error,
        path::{Path, PathBuf},
        str::FromStr,
    };
    use tempfile::tempdir;

    fn create_fake_repo_files(temp: &mut PathBuf) -> Result<(), Error> {
        fs::create_dir(&temp)?;

        temp.push("home");
        fs::create_dir(&temp)?;
        temp.push("dotfiles");
        fs::create_dir(&temp)?;

        // create home configs
        let mut home = temp.clone();
        home.push("HOME");
        fs::create_dir(&home)?;
        home.push(".config");
        fs::create_dir(&home)?;
        home.push("fish");
        fs::create_dir(&home)?;
        home.push("config.fish");
        File::create(&home)?;
        home.pop();
        home.pop();
        home.pop();
        home.push(".gitconfig");
        File::create(home)?;

        // create root configs
        temp.push("ROOT");
        fs::create_dir(&temp)?;
        temp.push("var");
        fs::create_dir(&temp)?;
        temp.push("etc");
        fs::create_dir(&temp)?;
        temp.push("config");
        fs::create_dir(&temp)?;
        temp.push("emacs.conf");
        File::create(temp)?;

        Ok(())
    }

    #[test]
    fn test_walkdir_walks_through_all_the_files() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let mut temp = temp_dir.path().to_path_buf();
        temp.push("walkdir_walks_through_all_the_files");
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("home");
        temp.push("dotfiles");

        let mut files = walk_repo(temp)
            .into_iter()
            .map(|f| f.display().to_string())
            .collect::<Vec<_>>();
        files.sort();
        let expected = vec![
            "HOME/.config/fish/config.fish",
            "HOME/.gitconfig",
            "ROOT/var/etc/config/emacs.conf",
        ];
        let diffs: Vec<_> = expected
            .into_iter()
            .zip(&files)
            .filter(|pair| pair.0 != *pair.1)
            .collect();
        assert_eq!(diffs.len(), 0);
        temp_dir.close()?;
        Ok(())
    }

    #[test]
    fn test_resolve_path_will_resolve_home_and_root_in_paths() {
        env::set_var("HOME", "/home/apollo");

        let Some(got) = resolve_path(PathBuf::from_str("HOME/.config/alacritty/alacritty.yml").unwrap()) else { panic!() };
        let expect = PathBuf::from_str("/home/apollo/.config/alacritty/alacritty.yml").unwrap();
        assert_eq!(expect, got);

        let Some(got) = resolve_path(PathBuf::from_str("HOME/.gitconfig").unwrap()) else { panic!() };
        let expect = PathBuf::from_str("/home/apollo/.gitconfig").unwrap();
        assert_eq!(expect, got);

        let Some(got) = resolve_path(PathBuf::from_str("ROOT/var/etc/conf/httpd.conf").unwrap()) else { panic!() };
        let expect = PathBuf::from_str("/var/etc/conf/httpd.conf").unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_create_symlinks_will_create_symlinks_where_required() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let mut temp = temp_dir.path().to_path_buf();
        temp.push("create_symlinks_where_required");
        let mut home = temp.clone();
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("home");
        temp.push("dotfiles");
        home.push("home");

        env::set_var("HOME", home.display().to_string());

        home.push(".gitconfig");
        unixfs::symlink(&temp.join("HOME").join(".gitconfig"), home.as_path())?;
        home.pop();

        let files = walk_repo(temp)
            .into_iter()
            .filter(|f| !f.starts_with("ROOT"));
        create_symlinks(files.collect())?;
        thread::sleep(Duration::from_millis(10));

        let expected_syms = vec![
            home.join(".gitconfig"),
            home.join(".config").join("fish").join("config.fish"),
        ];
        for file in expected_syms {
            assert!(file.is_symlink(), "{} is not a symlink", file.display());
        }
        temp_dir.close()?;
        Ok(())
    }

    #[test]
    fn test_throws_error_if_a_file_already_exists() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let mut temp = temp_dir.path().to_path_buf();
        temp.push("error_if_file_already_exists");
        let mut home = temp.clone();
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("home");
        temp.push("dotfiles");
        home.push("home");

        env::set_var("HOME", home.display().to_string());

        home.push(".gitconfig");
        File::create(home.clone())?;
        home.pop();

        let files = walk_repo(temp)
            .into_iter()
            .filter(|f| !f.starts_with("ROOT"));

        use std::io::{Error, ErrorKind};
        match create_symlinks(files.collect()) {
            Ok(()) => Err(Error::new(
                ErrorKind::Other,
                "Expected to throw error since .gitconfig should already exist",
            )),
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists
                    && e.to_string().contains("gitconfig already exists")
                {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[test]
    fn test_create_symlinks_will_error_if_current_symlink_is_wrong() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let mut temp = temp_dir.path().to_path_buf();
        temp.push("error_if_current_symlink_is_wrong");
        let mut home = temp.clone();
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("home");
        temp.push("dotfiles");
        home.push("home");

        env::set_var("HOME", home.display().to_string());

        home.push(".gitconfig");
        unixfs::symlink(Path::new("/fake_location/.gitconfig"), home.as_path())?;
        home.pop();

        let files = walk_repo(temp)
            .into_iter()
            .filter(|f| !f.starts_with("ROOT"));

        use std::io::{Error, ErrorKind};
        match create_symlinks(files.collect()) {
            Ok(()) => Err(Error::new(
                ErrorKind::Other,
                "Expected to throw error since .gitconfig points to wrong location",
            )),
            Err(e) => {
                if e.kind() == ErrorKind::Other
                    && e.to_string().contains("gitconfig linked to wrong file")
                {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}

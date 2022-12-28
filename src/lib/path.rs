use crate::git::dotfiles_dir;
use std::{env, fs, path::PathBuf, str::FromStr};
use std::os::unix::fs as unixfs;
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

fn resolve_path(path: PathBuf) -> Option<PathBuf> {
    if let Ok(file) = path.strip_prefix("HOME") {
        if let Ok(home_var) = env::var("HOME") {
            Some(PathBuf::from_str(&home_var).unwrap().join(file))
        } else {
            Some(PathBuf::from_str("/").unwrap().join(file))
        }
    } else {
        match path.strip_prefix("ROOT") {
            Ok(f) => Some(PathBuf::from_str("/").unwrap().join(f)),
            Err(_) => None,
        }
    }
}

fn create_symlinks(paths: Vec<PathBuf>) -> std::io::Result<()> {
    for path in paths {
        if let Some(sym_path) = resolve_path(path.to_path_buf()) {
            if let Some(sym_dir) = sym_path.parent() {
                fs::create_dir_all(sym_dir)?;
            }
            let path = PathBuf::from_str(&dotfiles_dir()).unwrap().join(path);
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
    use std::{env, io::Error, path::PathBuf, str::FromStr};
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_fake_repo_files(temp: &mut PathBuf) -> Result<(), Error> {
        temp.push("repo_files");
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
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("repo_files");

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
        let mut home = temp.clone();
        create_fake_repo_files(&mut temp.clone())?;
        temp.push("repo_files");
        home.push("home");

        env::set_var("HOME", home.display().to_string());
        fs::create_dir(&home)?;

        let files = walk_repo(temp)
            .into_iter()
            .filter(|f| !f.starts_with("ROOT"));
        create_symlinks(files.collect())?;

        let expected_syms = vec![
            home.join(".gitconfig"),
            home.join(".config").join("fish").join("config.fish"),
        ];
        for file in expected_syms {
            assert!(file.is_symlink());
        }
        Ok(())
    }
}

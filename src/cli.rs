use clap::{Parser, Subcommand};

/// Automatic dotfile configuration and backup using git
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}
/// Setup the dotfile folder by cloning from the repo

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Setup the dotfile folder by cloning from the repo
    Init {
        /// URL to repository
        url: String,
    },

    /// Create a new commit and push to the repo
    Push {
        /// Branch to commit and push to
        #[arg(default_value = "main")]
        branch: String,

        /// Commit message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Pull the changes from the repo and merge into current branch
    Pull {
        /// Branch to pull from
        #[arg(default_value = "main")]
        branch: String,
    },

    /// Adds a file to the dotfiles directory and replaces the file with a symlink
    Add {
        /// File to add to the dotfiles directory
        file: String,
    },

    /// Setup the config file symlinks
    Setup {},

    /// Call git status
    Status {},

    /// Call git log
    Log {},
}

pub fn parse() -> crate::cli::Args {
    Args::parse()
}

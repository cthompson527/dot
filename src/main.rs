use clap::{Parser, Subcommand};
use chrono::prelude::*;

mod git;

/// Automatic dotfile configuration and backup using git
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action
}
    /// Setup the dotfile folder by cloning from the repo

#[derive(Subcommand, Debug)]
enum Action {
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
        branch: String
    }
}

fn main() {
    let cli = Args::parse();
    println!("{:?}", cli);

    match &cli.action {
        Action::Init { url } => {
            println!("Pulling from {}", url);
            git::gclone(url.to_string());

        },
        Action::Push { branch, message } => {
            println!("Pushing to {}", branch);
            let msg = match message {
                Some(m) => m.to_owned(),
                None => format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S")),
            };
            git::gpush(branch.to_string(), msg);
        },
        Action::Pull { branch } => {
            println!("Pulling from {}", branch);
        },
    }

    // println!("{:?}", cli);
}

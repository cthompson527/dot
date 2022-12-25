use chrono::prelude::*;
use dot::git;

mod cli;

fn main() {
    let args = cli::parse();

    match &args.action {
        cli::Action::Init { url } => {
            println!("Pulling from {}", url);
            git::gclone(url.to_string());

        },
        cli::Action::Push { branch, message } => {
            println!("Pushing to {}", branch);
            let msg = match message {
                Some(m) => m.to_owned(),
                None => format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S")),
            };
            git::gpush(branch.to_string(), msg);
        },
        cli::Action::Pull { branch } => {
            println!("Pulling from {}", branch);
            git::gpull(branch.to_string());
        },
    }
}

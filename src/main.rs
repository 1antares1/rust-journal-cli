use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

/**
 * Own
 */
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file
    } = CommandLineArgs::from_args();

    /* Debug with pre-defined args
        let CommandLineArgs {
            action,
            journal_file
        } = CommandLineArgs {
            action: Add { text: String::from("Don't sell your Bitcoin on the cheap to Saylor") },
            journal_file: Some(PathBuf::from(r"E:\\rust-todo\\journal_file.json"))
        };
    */

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or_else(|| anyhow!("Failed to find journal file!"))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())

    // print only CLI options (panicked)
    // cli::CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".my-rusty-journal.json");
        path
    })
}

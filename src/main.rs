use clap::{Arg, ArgMatches, Command, value_parser};
use colored::Colorize;
use std::{env, error::Error, path::PathBuf};
mod notes;
mod storage;

fn get_storage_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let dir = exe_path.parent().ok_or("Failed to get content directory")?;
    let mut path = PathBuf::from(dir);
    path.push("notes.json");
    return Ok(path);
}

fn add(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let content = matches
        .get_one::<String>("content")
        .expect("`content` should always be present as it is required")
        .clone();

    let mut notes = notes::Notes::new(get_storage_path()?)?;
    notes.add(content);
    notes.save()?;

    return Ok(());
}

fn list() -> Result<(), Box<dyn Error>> {
    let notes = notes::Notes::new(get_storage_path()?)?;
    notes.list();

    return Ok(());
}

fn delete(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let idx = matches
        .get_one::<usize>("idx")
        .expect("`idx` should always be present as it is required");

    let mut notes = notes::Notes::new(get_storage_path()?)?;
    notes.delete(*idx)?;
    notes.save()?;

    return Ok(());
}

fn main() {
    let command = Command::new("ox")
        .about("Lightweight, Rust-powered CLI tool for managing notes.")
        .version("0.1.1")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a note")
                .visible_alias("new")
                .visible_alias("a")
                .arg(Arg::new("content").required(true).help("Note content")),
        )
        .subcommand(
            Command::new("list")
                .about("List notes")
                .visible_alias("ls")
                .visible_alias("l"),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a note")
                .visible_alias("remove")
                .visible_alias("rm")
                .visible_alias("d")
                .arg(
                    Arg::new("idx")
                        .required(true)
                        .help("Note index")
                        .value_parser(value_parser!(usize)),
                ),
        );

    let matches = command.get_matches();
    let result = match matches.subcommand() {
        Some(("add", sub_matches)) => add(sub_matches),
        Some(("list", _sub_matches)) => list(),
        Some(("delete", sub_matches)) => delete(sub_matches),
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "Error".bright_red(), e);
        return;
    }
}

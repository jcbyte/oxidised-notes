use clap::{Arg, ArgMatches, Command, value_parser};
use std::env;
mod notes;
mod storage;

fn get_storage_filename() -> String {
    let mut path = env::current_exe().unwrap().parent().unwrap().to_path_buf(); // todo make this safe
    path.push("notes.json");
    return path.to_str().unwrap().to_string();
}

fn add(matches: &ArgMatches) {
    let content = matches
        .get_one::<String>("content")
        .expect("`content` should always be present as it is required")
        .clone();

    let mut notes = notes::Notes::new(get_storage_filename());
    notes.add(content)
}

fn list() {
    let notes = notes::Notes::new(get_storage_filename());
    notes.list();
}

fn delete(matches: &ArgMatches) {
    let idx = matches
        .get_one::<usize>("idx")
        .expect("`idx` should always be present as it is required");

    let mut notes = notes::Notes::new(get_storage_filename());

    if let Err(e) = notes.delete(idx - 1) {
        eprintln!("{}", e);
    }
}

fn main() {
    let command = Command::new("Oxidised")
        .about("Lightweight, Rust-powered CLI tool for managing notes")
        .version("0.1.1")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a note")
                .arg(Arg::new("content").required(true).help("Note content")),
        )
        .subcommand(Command::new("list").about("List notes"))
        .subcommand(
            Command::new("delete").about("Delete a note").arg(
                Arg::new("idx")
                    .required(true)
                    .help("Note index")
                    .value_parser(value_parser!(usize)),
            ),
        );

    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => add(sub_matches),
        Some(("list", _sub_matches)) => list(),
        Some(("delete", sub_matches)) => delete(sub_matches),
        _ => unreachable!(),
    }
}

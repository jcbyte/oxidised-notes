use clap::{Arg, ArgMatches, Command, value_parser};

mod notes;
mod storage;

fn add(matches: &ArgMatches) {
    let content = matches
        .get_one::<String>("content")
        .expect("`content` should always be present as it is required");

    let mut notes = notes::Notes::new();
    notes.add(content)
}

fn list() {
    let notes = notes::Notes::new();
    notes.list();
}

fn delete(matches: &ArgMatches) {
    let idx = matches
        .get_one::<usize>("idx")
        .expect("`idx` should always be present as it is required");

    let mut notes = notes::Notes::new();
    notes.delete(idx - 1);
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

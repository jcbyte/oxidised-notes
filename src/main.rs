use clap::{Arg, ArgMatches, Command, value_parser};

// todo add notes
// todo view notes
// todo edit a note
// todo delete note

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
            Command::new("edit").about("Edit a note").arg(
                Arg::new("id")
                    .required(true)
                    .help("Note id")
                    .value_parser(value_parser!(u8)),
            ),
        )
        .subcommand(
            Command::new("delete").about("Delete a note").arg(
                Arg::new("id")
                    .required(true)
                    .help("Note id")
                    .value_parser(value_parser!(u8)),
            ),
        );
}

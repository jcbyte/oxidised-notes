use clap::{Arg, ArgMatches, Command, value_parser};

mod notes;
mod storage;

fn add(matches: &ArgMatches) {
    let content = matches
        .get_one::<String>("content")
        .expect("`content` should always be present as it is required");

    notes::add();
}

fn list(matches: &ArgMatches) {
    notes::list();
}

fn edit(matches: &ArgMatches) {
    let id = matches
        .get_one::<u8>("id")
        .expect("`id` should always be present as it is required");

    notes::edit();
}

fn delete(matches: &ArgMatches) {
    let id = matches
        .get_one::<u8>("id")
        .expect("`id` should always be present as it is required");

    notes::delete();
}

fn main() {
    let notes: Vec<String> = vec!["a", "b", "c"];
    storage::save(&notes);

    // let command = Command::new("Oxidised")
    //     .about("Lightweight, Rust-powered CLI tool for managing notes")
    //     .version("0.1.1")
    //     .arg_required_else_help(true)
    //     .subcommand(
    //         Command::new("add")
    //             .about("Add a note")
    //             .arg(Arg::new("content").required(true).help("Note content")),
    //     )
    //     .subcommand(Command::new("list").about("List notes"))
    //     .subcommand(
    //         Command::new("edit").about("Edit a note").arg(
    //             Arg::new("id")
    //                 .required(true)
    //                 .help("Note id")
    //                 .value_parser(value_parser!(u8)),
    //         ),
    //     )
    //     .subcommand(
    //         Command::new("delete").about("Delete a note").arg(
    //             Arg::new("id")
    //                 .required(true)
    //                 .help("Note id")
    //                 .value_parser(value_parser!(u8)),
    //         ),
    //     );

    // let matches = command.get_matches();
    // match matches.subcommand() {
    //     Some(("add", sub_matches)) => add(sub_matches),
    //     Some(("list", sub_matches)) => list(sub_matches),
    //     Some(("edit", sub_matches)) => edit(sub_matches),
    //     Some(("delete", sub_matches)) => delete(sub_matches),
    //     _ => unreachable!(),
    // }
}

mod db;
mod model;
use clap::{App, Arg, SubCommand};
use db::ToteDb;
use prettytable::{cell, row, Table};

fn main() -> Result<(), mysql::error::Error> {
    let mut tote_db = ToteDb::initialise()?;
    let matches = App::new("Tote")
        .version("1.0")
        .author("Omprakash Sridharan")
        .subcommand(SubCommand::with_name("ls").about("List totes"))
        .subcommand(
            SubCommand::with_name("add").about("Add tote").arg(
                Arg::with_name("note")
                    .short("n")
                    .value_name("Note")
                    .takes_value(true)
                    .help("Enter note to add"),
            ),
        )
        .get_matches();
    match matches.subcommand {
        Some(sc) => {
            let sc_name = &sc.name[..];
            match sc_name {
                "ls" => {
                    let mut table = Table::new();
                    let totes = tote_db.get_totes()?;
                    for tote in totes {
                        table.add_row(row![tote.id, tote.note]);
                    }
                    table.printstd();
                }
                "add" => match sc.matches {
                    arg_matches => match arg_matches.value_of("note") {
                        Some(val) => tote_db.add_tote(val.to_string())?,
                        None => println!("Invalid add command argument"),
                    },
                },
                _ => println!("Invalid subcommand"),
            }
        }
        None => println!("No sub command specified"),
    }

    //
    Ok(())
}

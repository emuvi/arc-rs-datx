use clap::{App, Arg, ArgMatches};

pub fn parse() -> ArgMatches {
  App::new("datx")
    .version(clap::crate_version!())
    .about("Datx (Data Base Toolbox) is a library and a command program that features a toolbox with a series of data base functionalities.")
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .arg(
      Arg::new("base")
        .short('b')
        .long("base")
        .takes_value(true)
        .value_name("URL")
        .required(true)
        .help("The URL connection for the database.")
    )
    .arg(
      Arg::new("export_to_csv")
        .long("export_to_csv")
        .takes_value(true)
        .value_name("DIR")
        .help("Export all tables to CSV files on the DIR argument.")
    )
    .get_matches()
}

extern crate clap;

use clap::{App, Arg};

fn main() {
    println!("RUSTACEAN!!");
    let matches = App::new("AgentCore")
    .arg(
        Arg::new("suggestions")
            .about("Returns suggestions for the string.")
            .short('s')
            .long("suggestions")
    )
    .after_help("Agent Core is meant to be embedded into your apps!")
    .get_matches();
    if matches.is_present("suggestions") {
        println!("You've enabled suggestions!")
    }
}
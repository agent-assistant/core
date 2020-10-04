extern crate clap;

/// Implement suggestions.
#[path = "src/suggestions.rs"] mod suggestions;

use clap::{App, Arg};

fn main() {
    let argapp = App::new("AgentCore")
    .name("Agent Core")
    .about(concat!("Agent Core is the centerpiece for Agent, the on-device virtual assistant. ",
    "The core features of Agent can be tested here, and you can use this in scripts.\n",
    "It's recommended, however, to use interop with another language so that you can ",
    "build it into an app or system UI."))
    .arg(
        Arg::new("suggestions")
            .about("Returns suggestions")
            .short('s')
            .long("suggestions")
            .conflicts_with_all(&["search", "askagent"])
    )
    .arg(
        Arg::new("search")
            .about("Returns search results and quick completions")
            .short('S')
            .long("search")
            .conflicts_with_all(&["suggestions", "askagent"])
    )
    .arg(
        Arg::new("askagent")
            .about("Returns the response from Agent")
            .short('q')
            .long("askagent")
            .conflicts_with_all(&["search", "suggestions"])
            .alias("ask")
            .alias("query")
    )
    .arg(
        Arg::new("input")
            .about("The string to use")
            .short('i')
            .long("input")
            .takes_value(true)
            .required(true)
    )
    .arg(
        Arg::new("wordfile")
            .about("A file with newline-separated words to use in corrections.")
            .requires("suggestions")
            .short('w')
            .long("wordfile")
            .alias("words")
            .takes_value(true)
            .conflicts_with("worddir")
    )
    .arg(
        Arg::new("worddir")
            .about("A directory where each file contains newline-separated words to use in corrections.")
            .requires("suggestions")
            .short('W')
            .long("worddir")
            .takes_value(true)
            .conflicts_with("wordfile")
    )
    //.after_help("Agent Core is meant to be embedded into your apps!")
    .after_help(concat!("Agent Core is built in Rust so that you can embed Agent into your app or device. ",
    "This does NOT enable you to interact with your app via existing Agent. Rather, this allows you to embed Agent ",
    "itself into your app, so you can take advantage of, for example, Agent Suggestions.\n",
    "Agent Core is also used to build the Agent app, Agent Search, and Agent Suggestions."));
    let matches = argapp.get_matches();
    if matches.is_present("suggestions") {
        //println!("You've asked for suggestions (using: {})!", matches.value_of("input").unwrap_or(""));
        println!("{}", suggestions::parse(matches.value_of("input").unwrap_or("")))
    } else if matches.is_present("search") {
        println!("You've asked for search results (using: {})!", matches.value_of("input").unwrap_or(""));
    } else if matches.is_present("askagent") {
        println!("You've asked Agent for something (using: {})!", matches.value_of("input").unwrap_or(""));
    } else {
        eprintln!("You must use suggestions, search, or askagent flags.")
    }
}


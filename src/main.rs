extern crate clap;

use clap::{Arg, App};
use std::fs;

fn main() -> std::io::Result<()> {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Roman Frołow <rofrol@gmail.com>")
        .about("Elm scaffolding in Rust")
        .arg(Arg::with_name("NAME")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("project name"))
        .get_matches();
    let name = matches.value_of("NAME").unwrap();
    println!("{}", name);

    fs::create_dir(name)?;
    Ok(())
}
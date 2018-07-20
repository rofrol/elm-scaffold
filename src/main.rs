extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;

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

    let root = Path::new(name);
    env::set_current_dir(&root)?;

    let output = Command::new("elm")
        .arg("package")
        .arg("install")
	.arg("-y")
        .output()
        .expect("'elm package install -y' command failed to start");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
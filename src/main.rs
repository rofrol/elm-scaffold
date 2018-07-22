#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};
use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ElmPackage {
    version: String,
    summary: String,
    repository: String,
    license: String,
    source_directories: Vec<String>,
    exposed_modules: Vec<String>,
    dependencies: HashMap<String, String>,
    elm_version: String,
}

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

    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
    println!("json: \n{}", json);


    let data = fs::read_to_string("elm-package.json").expect("Unable to read file");
    println!("data:\n {}", data);

    let data_json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("data_json: \n{}", serde_json::to_string_pretty(&data_json).unwrap());

    let data_json2 =
        serde_json::from_str::<ElmPackage>(&data).expect("JSON was not well-formatted");
    println!("data_json2: \n{}", serde_json::to_string_pretty(&data_json2).unwrap());

    Ok(())
}

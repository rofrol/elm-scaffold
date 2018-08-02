#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use std::path::Path;
use std::process::Command;

use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

extern crate fs_extra;
use fs_extra::dir;

use std::fs::OpenOptions;
use std::io::Write;

use std::env;
use std::io::{Seek, SeekFrom};

use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ElmPackage {
    version: String,
    summary: String,
    repository: String,
    license: String,
    source_directories: Vec<String>,
    exposed_modules: Vec<String>,
    #[serde(serialize_with = "ordered_map")]
    dependencies: HashMap<String, String>,
    elm_version: String,
}

fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

fn main() -> Result<(), Box<Error>> {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Roman Frołow <rofrol@gmail.com>")
        .about("Elm scaffolding in Rust")
        .arg(
            Arg::with_name("PROJECT")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("project name"),
        )
        .arg(
            Arg::with_name("TEMPLATE")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("template name"),
        )
        .get_matches();
    let project = matches.value_of("PROJECT").unwrap();
    println!("project name: {}", project);

    if Path::new(&project).exists() {
        eprintln!("error: path {:?} exists", &project);
        ::std::process::exit(1);
    }

    let template = matches.value_of("TEMPLATE").unwrap();
    println!("template path: {}", template);

    let root = env::var("CARGO_MANIFEST_DIR").expect("env var CARGO_MANIFEST_DIR not set?");

    let src = Path::new(&root).join(&template);
    let dst = Path::new(&root).join(&project);

    let mut dir_options = dir::CopyOptions::new();
    dir_options.copy_inside = true;
    dir::copy(&src, &dst, &dir_options)?;

    let output = Command::new("elm")
        .arg("package")
        .arg("install")
        .arg("-y")
        .current_dir(&dst)
        .output()?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(dst.join("elm-package.json"))?;

    let mut elm_package: ElmPackage = serde_json::from_reader(&file)?;

    elm_package.source_directories = vec!["src".to_owned()];

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    elm_package.serialize(&mut ser)?;
    let out = String::from_utf8(ser.into_inner()).unwrap();
    println!("elm_package with 4 space indentation: \n{}", out);
    // https://www.reddit.com/r/rust/comments/912h4l/stdfsopenoptionsnewwritetrueappendfalse_does_not/
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(out.as_bytes())?;

    Ok(())
}

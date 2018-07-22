#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

extern crate fs_extra;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

use std::fs::OpenOptions;
use std::io::Write;

use std::io::{Seek, SeekFrom};

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

fn main() -> std::io::Result<()> {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Roman Frołow <rofrol@gmail.com>")
        .about("Elm scaffolding in Rust")
        .arg(
            Arg::with_name("NAME")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("project name"),
        )
        .get_matches();
    let name = matches.value_of("NAME").unwrap();
    println!("{}", name);

    fs::create_dir(name)?;

    let dst = Path::new(name);

    let output = Command::new("elm")
        .arg("package")
        .arg("install")
        .arg("-y")
        .current_dir(dst)
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

    let options = CopyOptions::new(); //Initialize default values for CopyOptions

    let src = "templates/hello_world/src";
    copy(&src, &dst, &options).expect("copy failed");

    Ok(())
}

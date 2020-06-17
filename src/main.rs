extern crate clap;
extern crate clipboard;

use clap::{App, Arg};
use smallpaste::paste;
use std::process::Command;

fn main() {
    let matches = App::new("smallpaste")
        .version("0.1.0")
        .author("Kushal Das <mail@kushaldas.in>")
        .about("Uploads files to a personal pastebin.")
        .arg(
            Arg::with_name("public")
                .short("p")
                .help("Creates a public post.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("files")
                .help("The list of files")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let public = matches.is_present("public");

    let target = paste::create_local_path(public).unwrap();
    let a = matches.values_of("files");
    // For now we will only allow files
    let mut files = Vec::new();
    for file in a.unwrap() {
        files.push(file.to_string())
    }
    // now copy them to the righ place
    let mut files = paste::copy(target, files);

    let confs = paste::get_config_from_home();
    let conf = &confs[0];

    let _output = Command::new("/usr/bin/rsync")
        .arg("-avz")
        .arg("/tmp/volatile/")
        .arg(conf["output"].as_str().unwrap())
        .output();
    //println!("{:#?}", output.unwrap());
    let finalfile = files.pop().unwrap();
    let mut res = String::from(conf["url_prefix"].as_str().unwrap());
    let allchars = finalfile.chars();
    // TODO: find a better way to do this
    let mut i = 0;
    for ch in allchars {
        if ch == '/' {
            i += 1;
        }
        if i >= 3 {
            res.push(ch);
        }
    }
    println!("{}", res);
}

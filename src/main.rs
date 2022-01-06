use clap::{AppSettings, Parser, Subcommand};
use glob::glob;
use http::Method;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::{self, metadata, File},
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
    vec,
};
use walkdir::{DirEntry, WalkDir};

use crate::templates::{TEMPLATE_DELETE, TEMPLATE_GET, TEMPLATE_POST, TEMPLATE_PUT};

mod templates;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "KVZN <realkvz@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = 'd', long = "dir", required = true)]
    dir: String,
}

fn is_valid_file(entry: &DirEntry) -> bool {
    let is_file = entry.metadata().unwrap().is_file();

    let is_hidden = entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false);

    is_file && !is_hidden
}

fn list_files(dir: &str) -> Vec<String> {
    WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| {
            let e = e.unwrap();
            if is_valid_file(&e) {
                Some(e.path().display().to_string())
            } else {
                None
            }
        })
        .collect_vec()
}

fn grep_file(path: &str, substrings: Vec<&str>) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if substrings.iter().any(|&s| line.contains(&s)) {
                Some(line)
            } else {
                None
            }
        })
        .collect_vec()
}

fn process_line(line: &str) -> (Method, String) {
    let re = Regex::new(r#"#\[(\w+)\("(.+)"\)\]"#).unwrap();
    let caps = re.captures(line).unwrap();
    let method = match &caps[1] {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        _ => panic!("Unsupported HTTP method!"),
    };
    (method, caps[2].to_string())
}

fn process_lines(lines: &Vec<String>) -> Vec<(Method, String)> {
    lines.iter().map(|line| process_line(&line)).collect_vec()
}

fn main() {
    let opts: Opts = Opts::parse();

    println!(">>>>>>>>>>>>>>>> {:#?}", &opts);

    let files = list_files(&opts.dir);

    for file in files {
        // println!("----------------------------------- {}", &file);

        let lines = grep_file(&file, vec!["#[get(", "#[post(", "#[put(", "#[delete("]);

        let ls = process_lines(&lines);

        ls.iter()
            .group_by(|&(_method, path)| path)
            .into_iter()
            .map(|(path, items)| (path, items.map(|(method, p)| method).collect_vec()))
            .for_each(|(path, methods)| {
                println!(">>>>>>>>>> path: {}, methods: {:?}", &path, &methods);

                println!("  {}", &path);
                for method in methods {
                    match *method {
                        Method::GET => println!("{}", TEMPLATE_GET),
                        Method::POST => println!("{}", TEMPLATE_POST),
                        Method::PUT => println!("{}", TEMPLATE_PUT),
                        Method::DELETE => println!("{}", TEMPLATE_DELETE),
                        _ => panic!("Unsupported HTTP method!"),
                    }
                }
            });
    }
}

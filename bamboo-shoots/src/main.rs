extern crate regex;
use regex::Regex;
use std::env;
use std::path::Path;
use std::fs;

mod syntax;
use syntax::ownership;
use syntax::references;
use syntax::lifetimes;
use syntax::unsizedtype;

mod pointer;
use pointer::rc_arc;

mod parallel;
use parallel::threads;

fn main() {
    ownership::say_hello();
    references::say_hello();
    lifetimes::say_hello();
    unsizedtype::say_hello();
    rc_arc::say_hello();
    threads::say_hello();
}

pub fn read_file_names() {
    let re = Regex::new(r"s.").unwrap();
    let paths = fs::read_dir(&Path::new(&env::current_dir().unwrap())).unwrap();

    let names: Vec<String> = paths.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| {
                        let name = n.to_str().expect("error file name.");
                        if re.is_match(name) {
                            Some(String::from(name))
                        } else {
                            None
                        }
                    })
            })
        })
        .collect();

    println!("names: {:?}", names);
}

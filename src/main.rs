use std::env;
// use std::ffi::OsStr;
use rusqlite::{Connection, Result, Statement};
use std::path::Path;

#[derive(Debug)]
struct Prime {
    id: i64,
    value: i64,
}

fn get_program_name() -> String {
    let prog = env::args().next().unwrap();
    String::from(Path::new(&prog).file_name().unwrap().to_str().unwrap())
}
fn main() {}

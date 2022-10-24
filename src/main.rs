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
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut id_to_start: i64 = 0;
    let mut number_to_start: i64 = 0;
    let mut number_to_reach: i64 = 1_000_000_000;
    let mut reset_db: bool = false;

    if args.len() > 1 {
        for el in args.clone() {
            if el.contains("-h") || el.contains("--help") {
                println!("Usage: {} [Options]", get_program_name());
                println!("INTERVAL CHOICE:");
                println!("  -f=<number> / --from=<number> : Start number");
                println!("  -t=<number> / --to=<number>   : End number");
                println!("DATABASE:");
                println!("  -r / --reset-db   : Reset the database");
            }
            if el == "-r" || el == "--reset-db" {
                reset_db = true;
            }
        }
    }

    Ok(())
}

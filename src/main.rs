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
            if el.contains("-t=") || el.contains("--to=") {
                let to: Vec<&str> = el.split("=").collect();
                let k_to = to[0];
                let v_to: i64 = match to[1].trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Failed ! This option must have a number.");
                        -1
                    }
                };
                if v_to != -1 {
                    number_to_reach = v_to;
                }

                println!("{} {}", k_to, v_to)
            }
            if el == "-r" || el == "--reset-db" {
                reset_db = true;
            }
        }
    }

    let conn: Connection = Connection::open("p.db").unwrap();

    if reset_db {
        conn.execute("DROP TABLE IF EXISTS Primes;", ()).unwrap();
    }
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Primes (
             Id INTEGER,
             Prime INTEGER
         )",
        (),
    )
    .unwrap();

    let mut base_stmt: Statement =
        conn.prepare("SELECT Id, Prime FROM Primes ORDER BY Prime DESC LIMIT 1;")?;
    let base_iter = base_stmt
        .query_map([], |row| {
            Ok(Prime {
                id: row.get(0)?,
                value: row.get(1)?,
            })
        })
        .unwrap();
    for prime in base_iter {
        id_to_start = prime.unwrap().id;
    }

    let mut base_stmt: Statement =
        conn.prepare("SELECT Id, Prime FROM Primes ORDER BY Prime DESC LIMIT 1;")?;
    let base_iter = base_stmt
        .query_map([], |row| {
            Ok(Prime {
                id: row.get(0)?,
                value: row.get(1)?,
            })
        })
        .unwrap();

    for prime in base_iter {
        number_to_start = prime.unwrap().value;
    }

    println!("Last : {} (n°{})", id_to_start, number_to_start);

    Ok(())
}

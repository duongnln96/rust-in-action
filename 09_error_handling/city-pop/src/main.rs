extern crate getopts;

use getopts::Options;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::{env, path::Path};

// This struct represents the data in each row of the CSV file.
// Type based decoding absolves us of a lot of the nitty-gritty error
// handling, like parsing strings as integers or floats.
#[derive(Debug, Deserialize)]
struct Record {
    country: String,
    city: String,
    region: String,
    population: Option<u64>,
}

struct PopulationCount {
    city: String,
    country: String,
    // This is no longer an `Option` because values of this type are only
    // constructed if they have a population count.
    count: u64,
}

fn print_usage(program: &str, opts: Options) {
    println!(
        "{}",
        opts.usage(&format!("Usage: {} [options] <data-path> <city>", program))
    );
}

fn search<P: AsRef<Path>>(file_path: P, city: &str) -> Result<Vec<PopulationCount>, Box<Error>> {
    let mut found = vec![];
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.deserialize() {
        let record: Record = result?;

        match record.population {
            Some(count) => {
                if record.city == city {
                    found.push(PopulationCount {
                        city: record.city,
                        country: record.country,
                        count: count,
                    });
                }
            }
            None => {} // skip it
        }
    }

    if found.is_empty() {
        Err(From::from(
            "No matching cities with a population were found.",
        ));
    }
    Ok(found)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        "Choose an input file, instead of using STDIN.",
        "NAME",
    );
    opts.optflag("h", "help", "Show this usage message.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let data_path = matches.opt_str("f");

    let city = if !matches.free.is_empty() {
        &matches.free[0]
    } else {
        print_usage(&program, opts);
        return;
    };

    match search(data_path, city) {
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        }
        Err(err) => println!("{}", err),
    }
}

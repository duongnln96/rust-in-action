extern crate getopts;

use getopts::Options;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::{env, fmt, io, path::Path, process};

// This struct represents the data in each row of the CSV file.
// Type based decoding absolves us of a lot of the nitty-gritty error
// handling, like parsing strings as integers or floats.
#[derive(Debug, Deserialize)]
struct Record {
    country: String,
    city: String,
    population: Option<u64>,
}

struct PopulationCount {
    city: String,
    country: String,
    // This is no longer an `Option` because values of this type are only
    // constructed if they have a population count.
    count: u64,
}

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    Csv(csv::Error),
    NotFound,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::IoError(ref err) => err.fmt(f),
            CliError::Csv(ref err) => err.fmt(f),
            CliError::NotFound => write!(f, "No matching cities with a population were found."),
        }
    }
}

impl Error for CliError {
    // fn description(&self) -> &str {
    //     match *self {
    //         CliError::IoError(ref err) => err.description(),
    //         CliError::Csv(ref err) => err.description(),
    //         CliError::NotFound => "not found",
    //     }
    // }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            CliError::IoError(ref err) => Some(err),
            CliError::Csv(ref err) => Some(err),
            CliError::NotFound => None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        CliError::Csv(err)
    }
}

fn print_usage(program: &str, opts: Options) {
    println!(
        "{}",
        opts.usage(&format!("Usage: {} [options] <data-path> <city>", program))
    );
}

fn search<P: AsRef<Path>>(
    file_path: Option<P>,
    city: &str,
) -> Result<Vec<PopulationCount>, CliError> {
    let mut found = vec![];

    let input: Box<dyn io::Read> = match file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(File::open(file_path)?),
    };

    // let file = File::open(file_path.as_ref())?;
    let mut reader = csv::Reader::from_reader(input);

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
        Err(CliError::NotFound)
    } else {
        Ok(found)
    }
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
    opts.optflag("q", "quiet", "Silences errors and warnings.");

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

    match search(data_path.as_ref(), city) {
        Err(CliError::NotFound) if matches.opt_present("q") => process::exit(1),
        Err(err) => panic!("{}", err),
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        }
    }
}

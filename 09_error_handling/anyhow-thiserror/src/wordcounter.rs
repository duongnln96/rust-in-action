use std::error::Error;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

use thiserror::Error;

// Define enumerates error for word count using anyhow and thiserror
#[warn(dead_code)]
#[derive(Error, Debug)]
pub enum WordCountErrorLib {
    /// Represents an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// WordCountError implement manual
#[derive(Debug)]
pub enum WordCountError {
    EmptySource,
    ReadError { source: std::io::Error },
    IOError(std::io::Error),
}

impl fmt::Display for WordCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordCountError::EmptySource => write!(f, "Source contains no data"),
            WordCountError::IOError(ref err) => err.fmt(f),
            WordCountError::ReadError { source: ref err } => err.fmt(f),
        }
    }
}

impl Error for WordCountError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            WordCountError::EmptySource => None,
            WordCountError::IOError(ref err) => Some(err),
            // Our custom error doesn't have an underlying cause,
            // but we could modify it so that it does.
            WordCountError::ReadError { source: ref err } => Some(err),
        }
    }

    // fn cause(&self) -> Option<&dyn Error> {
    //     match *self {
    //         WordCountError::EmptySource => None,
    //         WordCountError::IOError(ref err) => Some(err),
    //         // Our custom error doesn't have an underlying cause,
    //         // but we could modify it so that it does.
    //         WordCountError::ReadError { source: ref err } => Some(err),
    //     }
    // }
}

impl From<std::io::Error> for WordCountError {
    fn from(err: std::io::Error) -> WordCountError {
        WordCountError::IOError(err)
    }
}

pub fn count_words<R: Read>(input: &mut R) -> Result<i32, WordCountError> {
    let reader = BufReader::new(input);
    let mut wordcount = 0;

    for line in reader.lines() {
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        for _word in line.split_whitespace() {
            wordcount += 1;
        }
    }

    if wordcount == 0 {
        return Err(WordCountError::EmptySource);
    }

    Ok(wordcount)
}

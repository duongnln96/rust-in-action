use std::io::{BufRead, BufReader, Read};

use thiserror::Error;

// Define enumerates error for word count
#[derive(Error, Debug)]
pub enum WordCountError {
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

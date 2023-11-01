pub mod wordcounter;

use std::env;
use std::fs::File;

use crate::wordcounter::count_words;
use anyhow::Context;

/// The run function is called from `main()` in [`rwc`].
///
/// It annotates errors from fallible functions (like `File::open` and
/// `count_words`) with `.context()` before propagating them upwards, where
/// `main()` will eventually handle reporting of the error.
pub fn run() -> anyhow::Result<()> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename).context(format!("unable to open '{}'", filename))?;
        let wordcount =
            count_words(&mut reader).context(format!("unable to count words in '{}'", filename))?;
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}

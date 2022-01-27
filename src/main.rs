use std::{path::PathBuf, fs, collections::HashMap};

use languages::{Language, LANGUAGES};
use structopt::StructOpt;

mod languages;
mod errors;
mod options;

use crate::options::Options;

type LanguageLookup = HashMap<Language, u64>;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Options::from_args();
    let mut usage_by_type: LanguageLookup = HashMap::new();
    traverse_path(&args.path, &mut usage_by_type);

    // calculate percentages of total
    // create graph
}

fn traverse_path(path: &PathBuf, lookup: &mut LanguageLookup) -> Option<()> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.is_file() {
        let filesize = metadata.len();
        let language = extract_filetype(&path);
        let total = lookup.entry(language).or_insert(0);
        *total += filesize;
    }

    for entry in fs::read_dir(path).ok()? {
        if let Ok(directory) = entry {
            traverse_path(&directory.path(), lookup);
        }
    }

    return traverse_path(path, lookup)
}

fn extract_filetype(path: &PathBuf) -> Language {
    // figure out which language a file is
}

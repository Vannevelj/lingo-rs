use languages::{get_extensions, Language};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};
use structopt::StructOpt;

mod errors;
mod languages;
mod options;

use crate::options::Options;

type LanguageLookup = HashMap<Language, u64>;

lazy_static! {
    static ref EXTENSIONS: Vec<Language> = get_extensions();
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Options::from_args();

    let mut usage_by_type: LanguageLookup = HashMap::new();
    println!("Starting now at {:?}", &args.path);
    traverse_path(&args.path, &mut usage_by_type, 0);

    let total_bytes: u64 = usage_by_type.values().sum();

    for (language, count) in usage_by_type {
        println!(
            "{}: {:.2}%",
            language.name,
            count as f64 / total_bytes as f64 * 100f64
        );
    }

    // create graph
}

fn traverse_path(path: &PathBuf, lookup: &mut LanguageLookup, depth: u8) -> Option<()> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.is_file() {
        println!("Inspecting {:?}", &path);
        if is_binary_file(path) {
            info!("Skipping binary file at {:?}", &path);
            return None;
        }

        let filesize = metadata.len();
        if let Some(language) = extract_filetype(&path) {
            let total = lookup.entry(language).or_insert(0);
            *total += filesize;
        }
    } else {
        if !should_skip_path(&path, depth) {
            for entry in fs::read_dir(path).ok()? {
                if let Ok(directory) = entry {
                    traverse_path(&directory.path(), lookup, depth + 1);
                }
            }
        }
    }

    return None;
}

fn extract_filetype(path: &PathBuf) -> Option<Language> {
    // figure out which language a file is
    let current_path_extension = path.extension()?.to_str()?.to_string();
    if let Some(language) = EXTENSIONS
        .iter()
        .find(|x| x.extensions.contains(&current_path_extension))
    {
        return Some(language.clone());
    }

    None
}

fn should_skip_path(path: &PathBuf, depth: u8) -> bool {
    let to_skip = vec!["node_modules", "build", "target", "bin", "obj", "generated"];
    if depth <= 2 {
        if let Some(path) = path.as_os_str().to_str() {
            let should_skip = to_skip.iter().any(|pattern| path.contains(pattern));
            if should_skip {
                info!("Skipping {:?}", path);
            }

            return should_skip;
        }
    }

    return false;
}

/*
    Git considers a file binary if there is a NUL character within the first 8000 bytes
    see: https://github.com/git/git/blob/9c9b961d7eb15fb583a2a812088713a68a85f1c0/xdiff-interface.c#L187-L193
*/
fn is_binary_file(path: &PathBuf) -> bool {
    let file = File::open(path);
    match file {
        Ok(mut file) => {
            let mut buffer = [0u8; 8000];
            buffer.fill(0);
            match file.read(&mut buffer) {
                Ok(num_bytes) => buffer[..num_bytes].contains(&b'\x00'),
                Err(e) => {
                    error!("Error: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            error!("Error: {:?}", e);
            false
        }
    }
}

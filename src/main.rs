use chrono::{Duration, NaiveDate, Utc};
use languages::{get_extensions, Language};
use lazy_static::lazy_static;
use log::{debug, error, info};

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    ops::Add,
    path::{Path, MAIN_SEPARATOR},
    process::{Command, Stdio},
    str,
};
use structopt::StructOpt;

mod graph;
mod languages;
mod options;

use crate::{graph::create_graph, options::Options};

type LanguageLookup = BTreeMap<Language, u64>;
type DistributionLookup = BTreeMap<NaiveDate, LanguageLookup>;

#[derive(Debug, Clone)]
pub struct Prevalence {
    percentage: f64,
    cumulative_percentage: f64,
}
type ChronologicalLookup = BTreeMap<Language, BTreeMap<NaiveDate, Prevalence>>;

lazy_static! {
    static ref EXTENSIONS: Vec<Language> = get_extensions();
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Options::from_args();
    info!("Starting now at {:?}", &args.path);

    let mut distribution_by_date: DistributionLookup = BTreeMap::new();
    let date_format = "%Y-%m-%d";
    let mut start = NaiveDate::parse_from_str(args.start.as_str(), date_format)
        .expect("Invalid start date provided");
    let mut end: NaiveDate = Utc::now().naive_utc().date();
    if let Some(d) = args.end {
        end = NaiveDate::parse_from_str(d.as_str(), date_format).expect("Failed to parse end date");
    }

    while start < end {
        info!("Evaluating {}", start);
        checkout_date(&start, &args.branch, &args.path);

        let mut usage_by_type: LanguageLookup = BTreeMap::new();

        traverse_path(&args.path, &mut usage_by_type);
        distribution_by_date.insert(start, usage_by_type);

        start = start.add(Duration::days(1));
    }

    reset_repo(&args.branch, &args.path);

    let mapped_data = rollup_data(distribution_by_date);
    create_graph(&mapped_data, args.name);
}

fn traverse_path(path: &Path, lookup: &mut LanguageLookup) -> Option<()> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.is_file() {
        debug!("Inspecting {:?}", &path);
        if is_binary_file(path) {
            debug!("Skipping binary file at {:?}", &path);
            return None;
        }

        let filesize = metadata.len();
        if let Some(language) = extract_filetype(path) {
            let total = lookup.entry(language).or_insert(0);
            *total += filesize;
        }
    } else if !should_skip_path(path) {
        for entry in (fs::read_dir(path).ok()?).flatten() {
            traverse_path(&entry.path(), lookup);
        }
    }

    None
}

fn extract_filetype(path: &Path) -> Option<Language> {
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

fn should_skip_path(path: &Path) -> bool {
    let to_skip = vec![
        "node_modules",
        "build",
        "target",
        "bin",
        "obj",
        "__generated__",
        "generated",
        "Pods",
        ".git",
        "Resources",
    ];
    if let Some(path) = path.as_os_str().to_str() {
        let should_skip = to_skip
            .iter()
            .any(|pattern| path.contains(&format!("{}{}", pattern, MAIN_SEPARATOR)));
        if should_skip {
            debug!("Skipping {:?}", path);
        }

        return should_skip;
    }

    false
}

/*
    Git considers a file binary if there is a NUL character within the first 8000 bytes
    see: https://github.com/git/git/blob/9c9b961d7eb15fb583a2a812088713a68a85f1c0/xdiff-interface.c#L187-L193
*/
fn is_binary_file(path: &Path) -> bool {
    let file = File::open(path);
    match file {
        Ok(mut file) => {
            let mut buffer = [0u8; 8000];
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

fn checkout_date(date: &NaiveDate, branch: &str, path: &Path) {
    let output = Command::new("git")
        .args([
            "rev-list",
            "-1",
            "--before",
            date.format("%Y-%m-%d").to_string().as_str(),
            branch,
        ])
        .stdout(Stdio::piped())
        .current_dir(&path)
        .output()
        .expect("Failed to get rev-list");

    let commit_hash = str::from_utf8(&output.stdout)
        .expect("Failed to parse commit hash")
        .trim();
    debug!("Commit hash: {}", commit_hash);

    Command::new("git")
        .args(["checkout", commit_hash])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(&path)
        .spawn()
        .expect("Failed to checkout date");
}

fn reset_repo(branch: &str, path: &Path) {
    Command::new("git")
        .args(["checkout", branch])
        .current_dir(path)
        .spawn()
        .expect("Failed to reset repository");
}

fn rollup_data(data: DistributionLookup) -> ChronologicalLookup {
    let mut language_map: ChronologicalLookup = BTreeMap::new();

    for (date, values) in data {
        let total_bytes: u64 = values.values().sum();
        let mut cumulative_percentage = 0.0;
        for (language, count) in values {
            let percentage = count as f64 / total_bytes as f64 * 100.0;
            cumulative_percentage += percentage;
            let prevalence = Prevalence {
                percentage,
                cumulative_percentage,
            };

            if let Some(language_map_entry) = language_map.get_mut(&language) {
                language_map_entry.insert(date, prevalence);
            } else {
                let mut new_language_map: BTreeMap<NaiveDate, Prevalence> = BTreeMap::new();
                new_language_map.insert(date, prevalence);
                language_map.insert(language.clone(), new_language_map);
            }

            debug!(
                "Evaluating {} on {}: {} ({})",
                language.name, date, percentage, cumulative_percentage
            );
        }
    }

    language_map
}

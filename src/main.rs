use chrono::{Duration, NaiveDate, Utc};
use languages::{get_extensions, Language};
use lazy_static::lazy_static;
use log::{debug, error, info};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    ops::Add,
    path::{PathBuf, MAIN_SEPARATOR},
    process::{Command, Stdio},
    str,
};
use structopt::StructOpt;

mod languages;
mod options;

use crate::options::Options;

type LanguageLookup = HashMap<Language, u64>;
type DistributionLookup = HashMap<NaiveDate, LanguageLookup>;

lazy_static! {
    static ref EXTENSIONS: Vec<Language> = get_extensions();
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Options::from_args();
    info!("Starting now at {:?}", &args.path);

    let mut distribution_by_date: DistributionLookup = HashMap::new();
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

        let mut usage_by_type: LanguageLookup = HashMap::new();

        traverse_path(&args.path, &mut usage_by_type);
        distribution_by_date.insert(start, usage_by_type);

        start = start.add(Duration::days(1));
    }

    remove_branch();

    // let total_bytes: u64 = usage_by_type.values().sum();

    // for (language, count) in usage_by_type {
    //     println!(
    //         "{}: {:.2}%",
    //         language.name,
    //         count as f64 / total_bytes as f64 * 100f64
    //     );
    // }

    // create graph
}

fn traverse_path(path: &PathBuf, lookup: &mut LanguageLookup) -> Option<()> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.is_file() {
        debug!("Inspecting {:?}", &path);
        if is_binary_file(path) {
            debug!("Skipping binary file at {:?}", &path);
            return None;
        }

        let filesize = metadata.len();
        if let Some(language) = extract_filetype(&path) {
            let total = lookup.entry(language).or_insert(0);
            *total += filesize;
        }
    } else {
        if !should_skip_path(&path) {
            for entry in fs::read_dir(path).ok()? {
                if let Ok(directory) = entry {
                    traverse_path(&directory.path(), lookup);
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

fn should_skip_path(path: &PathBuf) -> bool {
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
fn is_binary_file(path: &PathBuf) -> bool {
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

fn checkout_date(date: &NaiveDate, branch: &String, path: &PathBuf) {
    // git checkout `git rev-list -1 --before "$startdate" "$branch"`
    // let rev_list_command = &format!("git rev-list -1 --before \"{}\" \"{}\"", date.format("%Y-%m-%d"), branch);

    println!(
        "{:?}",
        Command::new("git")
            .args([
                "rev-list",
                "-1",
                "--before",
                date.format("%Y-%m-%d").to_string().as_str(),
                branch,
            ])
            .current_dir(&path)
            .get_args()
    );

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

    let commit_hash = str::from_utf8(&output.stdout).expect("Failed to parse commit hash");
    info!("Commit hash: {}", commit_hash);

    Command::new("git")
        .args(["checkout", commit_hash])
        .spawn()
        .expect("Failed to checkout date");
}

fn remove_branch() {
    Command::new("git")
        .args(&["branch", "-D", "lingo-rs"])
        .spawn()
        .expect("Failed to clean up branch date");
}

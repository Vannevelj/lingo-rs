use chrono::{Duration, NaiveDate, Utc};
use directories::UserDirs;
use languages::{get_extensions, Language};
use lazy_static::lazy_static;
use log::{debug, error, info};
use plotters::{
    prelude::{
        BitMapBackend, ChartBuilder, Circle, EmptyElement, IntoDrawingArea, LineSeries,
        PointSeries, Text,
    },
    style::{IntoFont, RED, WHITE},
};
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

    // while start < end {
    //     info!("Evaluating {}", start);
    //     checkout_date(&start, &args.branch, &args.path);

    //     let mut usage_by_type: LanguageLookup = HashMap::new();

    //     traverse_path(&args.path, &mut usage_by_type);
    //     distribution_by_date.insert(start, usage_by_type);

    //     start = start.add(Duration::days(1));
    // }

    // reset_repo(&args.branch, &args.path);
    create_graph(&distribution_by_date);
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
        .current_dir(&path)
        .spawn()
        .expect("Failed to checkout date");
}

fn reset_repo(branch: &String, path: &PathBuf) {
    Command::new("git")
        .args(["checkout", branch])
        .current_dir(path)
        .spawn()
        .expect("Failed to reset repository");
}

// let total_bytes: u64 = usage_by_type.values().sum();

// for (language, count) in usage_by_type {
//     println!(
//         "{}: {:.2}%",
//         language.name,
//         count as f64 / total_bytes as f64 * 100f64
//     );
// }

fn create_graph(data: &DistributionLookup) {
    let output_file = UserDirs::new()
        .expect("Could not find a HOME directory")
        .desktop_dir()
        .expect("No Desktop directory found")
        .join("out.png");
    println!("{:?}", UserDirs::new().unwrap().desktop_dir().unwrap());
    println!("{:?}", output_file);
    let root = BitMapBackend::new(&output_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10f32, 10f32, 10f32, 10f32);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20f32)
        .y_label_area_size(40f32)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)
        .unwrap();

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();

    // And we can draw something in the drawing area
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
            &RED,
        ))
        .unwrap();
    // Similarly, we can draw point series
    chart
        .draw_series(PointSeries::of_element(
            vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
            },
        ))
        .unwrap();
}

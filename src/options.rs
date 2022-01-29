use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Directory to evaluate
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Start date from which to analyse
    #[structopt(short, long)]
    pub start: String,

    /// Start date from which to analyse
    #[structopt(short, long)]
    pub end: Option<String>,
}

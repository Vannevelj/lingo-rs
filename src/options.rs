use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Directory to evaluate
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Start date from which to analyse
    #[structopt(short, long)]
    pub start: String,

    /// Last date to use for analysis
    #[structopt(short, long)]
    pub end: Option<String>,

    /// The branch which to analyse
    #[structopt(default_value = "master", short, long)]
    pub branch: String,

    /// The name of the repository
    #[structopt(short, long)]
    pub name: String,
}

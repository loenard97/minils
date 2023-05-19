use std::error::Error;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use atty::Stream;
use clap::Parser;
use colorama::Colored;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub dir_name: Option<String>,
    #[arg(short, long)]
    pub all: bool,
    #[arg(short, long)]
    pub long: bool,
    #[arg(short, long)]
    pub tree: bool,
    #[arg(short, long)]
    pub pretty_print: Option<bool>,
}

pub struct Config {
    pub dir_name: String,
    pub all: bool,
    pub long: bool,
    pub tree: bool,
    pub pretty_print: bool,
    pub stdout_exists: bool,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        
        let dir_name = args.dir_name.unwrap_or(String::from("./"));
        let pretty_print = atty::is(Stream::Stdout) && args.pretty_print.is_none() || args.pretty_print.unwrap_or_default();
        let all = args.all;
        let long = args.long;
        let tree = args.tree;
        let stdout_exists = atty::is(Stream::Stdout);

        Config { dir_name, all, long, tree, pretty_print, stdout_exists }
    }
}

struct FileInfo {
    name: String,
    size: usize,
}

impl FileInfo {
    fn new(dir_entry: &DirEntry) -> Self {
        let metadata = fs::metadata(dir_entry.path()).unwrap();

        let name = String::from(dir_entry.file_name().to_str().unwrap());

        FileInfo { name, size: 0 }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(config.dir_name.clone()).unwrap();
    let n_items = fs::read_dir(config.dir_name.clone()).unwrap().count();

    let path_filter = if config.all {
        |_: &Result<DirEntry, std::io::Error>| { true }
    } else {
        |dir_entry: &Result<DirEntry, std::io::Error>| {
            let file_name = dir_entry.as_ref().unwrap().file_name();
            let file_name = String::from(file_name.to_str().unwrap());
            !file_name.starts_with('.')
        }
    };

    if config.pretty_print {
        println!("{}", head(&config.dir_name));
    }

    for (i, path) in paths.filter(path_filter).enumerate() {
        let is_last = i == n_items;

        // let dir_entry = path.unwrap();
        // let file_info = FileInfo::new(&dir_entry);

        // let path = dir_entry.path();
        // let line = path.to_str().unwrap().trim_matches(|c| c == '.' || c == '/');
        println!("{}", body(config, &path.unwrap().path(), 0, is_last));
    }

    Ok(())
}

fn head(file_name: &str) -> String {
    let mut result = String::new();
    let n_cols: usize = termsize::get().unwrap().cols.into();

    result.push_str(&format!("  Directory {}\n", file_name.to_string().style("bold")));
    result.push_str(&format!("{}", "─".repeat(n_cols)));

    result
}

fn body(config: &Config, path: &PathBuf, depth: u8, is_last: bool) -> String {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let mut file_name: String = file_name.trim_end_matches(char::is_whitespace).into();
    if path.is_dir() {
        file_name = file_name.color("bright blue");
    }

    let metadata = fs::metadata(path).unwrap();

    let mut result = String::new();

    if config.pretty_print {
        result.push_str("  ");
    }
    if config.pretty_print && config.tree {
        result.push_str(&format!(" │ "));
    }

    let s = format!("{}  {}", file_name, metadata.len());

    result.push_str(&s);

    result
}

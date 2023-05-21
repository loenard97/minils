use std::error::Error;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use atty::Stream;
use clap::Parser;
use colorama::Colored;
use cliform::{Grid, Tree, TreeStyle};

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
        let pretty_print = atty::is(Stream::Stdout) && args.pretty_print.unwrap_or_default();
        let all = args.all;
        let long = args.long;
        let tree = args.tree;
        let stdout_exists = atty::is(Stream::Stdout);

        Config { dir_name, all, long, tree, pretty_print, stdout_exists }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let mut output = String::new();

    let paths = fs::read_dir(config.dir_name.clone()).unwrap();
    let n_items = fs::read_dir(config.dir_name.clone()).unwrap().count();
    let n_cols: usize = termsize::get().unwrap().cols.into();

    let path_filter = if config.all {
        |_: &Result<DirEntry, std::io::Error>| { true }
    } else {
        |dir_entry: &Result<DirEntry, std::io::Error>| {
            let file_name = dir_entry.as_ref().unwrap().file_name();
            let file_name = String::from(file_name.to_str().unwrap());
            !file_name.starts_with('.')
        }
    };

    // head
    if config.pretty_print {
        output.push_str(&format!("  Directory {}\n", config.dir_name.to_string().style("bold")));
        output.push_str(&format!("{}", "─".repeat(n_cols)));
    }

    if !config.long && !config.tree {
        // grid
        let mut grid = Grid::new(2, 100);

        for entry in paths.filter(path_filter) {
            let file_name = entry.unwrap().file_name().to_str().unwrap().to_string();
            grid.push(file_name);
        }

        output.push_str(&grid.to_string());

    } else if config.long {
        // list
        let mut list = String::new();
    
        for entry in paths.filter(path_filter) {
            let file_name = entry.unwrap().file_name();
            list.push_str(file_name.to_str().unwrap());
            list.push('\n');
        }
    
        output.push_str(&list);
        
    } else if config.tree {
        // tree
        let mut tree = Tree::new();
    
        for (i, entry) in paths.filter(path_filter).enumerate() {
            let is_last = i == n_items;
            let file_name = entry.unwrap().file_name();
            tree.push(file_name.to_str().unwrap().to_string(), 0, is_last);
        }
    
        output.push_str(&tree.to_string(TreeStyle::Lines));
        
    } else {
        println!("other");
    }

    println!("{}", output);

    Ok(())
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

    if config.pretty_print {
        result.push_str(&format!("{}  {}", file_name, metadata.len()));
    } else {
        
    }

    result
}

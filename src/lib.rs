use std::error::Error;
use std::fs::{self, DirEntry};

use atty::Stream;
use clap::Parser;
use chrono::{DateTime, Local};

use colorama::Colored;
use cliform::{Grid, Table, Tree, TreeStyle};

mod util;

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
    pub pretty_print: bool,
}

#[derive(PartialEq)]
pub enum PrintConfig {
    Grid,
    List,
    Table,
    Tree,
}

pub struct Config {
    pub dir_name: String,
    print_config: PrintConfig,
    pub all: bool,
    pub pretty_print: bool,
    pub stdout_exists: bool,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        
        let dir_name = args.dir_name.unwrap_or(String::from("./"));
        let stdout_exists = atty::is(Stream::Stdout);
        let pretty_print = stdout_exists && args.pretty_print;
        let all = stdout_exists && args.all;
        let mut print_config = PrintConfig::Grid;
        if args.long { print_config = PrintConfig::Table; }
        if args.tree { print_config = PrintConfig::Tree; }
        if !stdout_exists { print_config = PrintConfig::List; }

        Config { dir_name, all, print_config, pretty_print, stdout_exists }
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

    if config.print_config == PrintConfig::Grid {
        let mut grid = Grid::new(2, 100);

        for entry in paths.filter(path_filter) {
            let file_name = entry.unwrap().file_name().to_str().unwrap().to_string();
            grid.push(file_name);
        }

        output.push_str(&grid.to_string());

    } else if config.print_config == PrintConfig::Table {
        let mut table: Table<String> = Table::new();
        table.header(vec![String::from("Last modified"), String::from("Size"), String::from("Name")]);
        let mut entry_info: Vec<String> = Vec::new();

        for entry in paths.filter(path_filter) {
            entry_info.clear();

            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_path = file_name.to_str().unwrap().to_string();

            let metadata = fs::metadata(file_name.clone()).unwrap();

            let system_time = metadata.modified().unwrap();
            let datetime: DateTime<Local> = system_time.into();
            entry_info.push(datetime.format("%Y/%m/%d %H:%M").to_string());

            if metadata.len() > 0 {
                entry_info.push(util::file_size_to_string(metadata.len()));
            } else {
                entry_info.push("".to_string());
            }

            entry_info.push(file_path);

            table.push(entry_info.clone());
        }

        output.push_str(&table.to_string(2));
        
    } else if config.print_config == PrintConfig::Tree {
        let mut tree = Tree::new();
    
        for (i, entry) in paths.filter(path_filter).enumerate() {
            let is_last = i == n_items;
            let file_name = entry.unwrap().file_name();
            tree.push(file_name.to_str().unwrap().to_string(), 0, is_last);
        }
    
        output.push_str(&tree.to_string(TreeStyle::Lines));
        
    } else if config.print_config == PrintConfig::List {
        for entry in paths.filter(path_filter) {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_path = file_name.to_str().unwrap().to_string();
            
            output.push_str(&file_path);
            output.push('\n');
        }

        output = output.trim_end_matches('\n').to_string();

    } else {
        panic!("unhandled config");
    }

    println!("{}", output);

    Ok(())
}

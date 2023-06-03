use std::fs::{self, DirEntry};
use chrono::{DateTime, Local};
use cliform::{Grid, Table, Tree};

mod util;

pub fn format_as_grid(paths: Vec<DirEntry>) -> String {
    let mut grid = Grid::new();
    let max_len = termsize::get()
        .unwrap_or(termsize::Size { rows: 100, cols: 50})
        .cols.into();

    for entry in paths {
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_else(|| { "<unknown>" }).to_string();
        grid.push(file_name);
    }

    grid.to_string(2, max_len)
}

pub fn format_as_table(paths: Vec<DirEntry>) -> String {
    let mut table: Table<String> = Table::new();
    table.header(vec![String::from("Last modified"), String::from("Size"), String::from("Name")]);
    let mut entry_info: Vec<String> = Vec::new();

    for entry in paths {
        entry_info.clear();

        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_else(|| { "<unknown>" }).to_string();

        let metadata = fs::metadata(file_name.clone());
        let last_modified = match &metadata {
            Ok(md) => match md.modified() {
                Ok(time) => {
                    let datetime: DateTime<Local> = time.into();
                    datetime.format("%Y/%m/%d %H:%M").to_string()
                },
                Err(_) => String::from("----"),
            },
            Err(_) => String::from("----"),
        };
        let size = match &metadata {
            Ok(md) => util::file_size_to_string(md.len()),
            Err(_) => String::from("----"),
        };

        entry_info.push(last_modified);
        entry_info.push(size);
        entry_info.push(file_name);
        table.push(entry_info.clone());
    }

    table.to_string(2)
}

pub fn format_as_tree(paths: Vec<DirEntry>) -> String {
    let mut tree = Tree::new();
    let n_items = paths.iter().count();
    
    for (i, entry) in paths.iter().enumerate() {
        let is_last = i == n_items;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_else(|| { "<unknown>" }).to_string();
        tree.push(file_name, 0, is_last);
    }

    tree.to_string()
}

pub fn format_as_list(paths: Vec<DirEntry>) -> String {
    let mut output = String::new();

    for entry in paths {
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_else(|| { "<unknown>" }).to_string();
        output.push_str(&file_name);
        output.push('\n');
    }

    output.trim_end_matches('\n').to_string()
}

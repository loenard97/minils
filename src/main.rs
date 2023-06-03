use std::process;
use std::fs;
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
    pub pretty_print: bool,
}

fn main() {
    let args = Args::parse();

    let dir = fs::read_dir(
        args.dir_name.clone()
        .unwrap_or(String::from("./")))
        .unwrap_or_else(|_| { 
            eprint!("{}: The given directory could not be accessed\n", String::from("Application error").color("red").style("bold"));
            process::exit(1)
    });

    let paths: Vec<_> = dir
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|entry| { 
            atty::is(Stream::Stdout) && args.all || { 
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap_or_default().to_string();
                !file_name.starts_with('.')} 
            })
        .collect();

    println!("{}", match (args, atty::is(Stream::Stdout)) {
        (Args { long: true, .. }, _) => minils::format_as_table(paths),
        (Args { tree: true, .. }, _) => minils::format_as_tree(paths),
        (_, false) => minils::format_as_list(paths),
        _ => minils::format_as_grid(paths),
    });
}

use clap::{App, Arg};
use std::io::{BufReader, BufRead};
use std::fs::{File, OpenOptions};
use dirs;
use std::path::{Path, PathBuf};

fn snipp_home_dir() -> PathBuf {
    let mut home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
    println!("$HOME: {}", home_dir.display());
    home_dir.push(".snipp");
    home_dir
}

fn create_dir(path: &Path) {
    println!("Create snipp dir...");
    match std::fs::create_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Create directory {}", path.display())
    }
}

fn create_snippet_file(dir_path: &Path) {
    println!("Create snippet file...");
    let path = dir_path.join(".snippets");
    match File::create(path.as_path()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Create snippet file {}", path.display())
    }
}

//fn load_snippet_file(file_path: &str) -> Result<Vec<String>, dyn std::error::Error> {
//    let file = File::open(file_path)?;
//    let result = BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect();
//    return Ok(result);
//}

fn main() {
    let matches = App::new("Snipp")
        .version("1.0.0")
        .author("kz_morita")
        .about("Simple snippet tool for CLI")
        .arg(
            Arg::with_name("verbose").short("v").long("verbose").required(false)
        ).get_matches();

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);

    // create snipp directory
    let dir_path = snipp_home_dir();
    create_dir(dir_path.as_path());

    // create snippet file
    create_snippet_file(dir_path.as_path());
}

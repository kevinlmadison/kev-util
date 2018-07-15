#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate appdirs;

use clap::App;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use walkdir::WalkDir;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let vflag = matches.is_present("verbose");
    write_verbose("Verbose output enabled.", &vflag);
    write_verbose(&format!("Cache Directory: {}", get_cache_dir()), &vflag);
    write_verbose(&format!("Cache File: {}", get_cache_file()), &vflag);
}

fn write_verbose(message: &str, v: &bool) {
    if *v {
        eprintln!("{}", message);
    }
}

fn get_cache_dir() -> String {
    let cache_dir = Path::new(
        &appdirs::user_data_dir(Some(""), None, false)
        .unwrap())
        .join("quartus-search");
    cache_dir.into_os_string().into_string().unwrap()
}

fn get_cache_file() -> String {
    let cache_file = Path::new(&get_cache_dir()).join("cache.json");
    cache_file.into_os_string().into_string().unwrap()
}

fn purge(v: &bool) {
    write_verbose("Purging cache... ", v);
    if Path::new(&get_cache_dir()).exists() {
        write_verbose("Begin deletion...", v);
        fs::remove_dir_all(Path::new(&get_cache_dir()));
        write_verbose("Cache purge complete", v);
    } else {
        write_verbose("Cache directory does not exist.", v);
    }
}

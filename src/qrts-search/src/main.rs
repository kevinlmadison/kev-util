#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate appdirs;

use clap::App;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
        fs::remove_dir_all(Path::new(&get_cache_dir())).unwrap();
        write_verbose("Cache purge complete", v);
    } else {
        write_verbose("Cache directory does not exist.", v);
    }
}

fn parse_version(entry: &walkdir::DirEntry, v: &bool) -> Option<String> {
    write_verbose("parsing version from nios2eds installation", v);

    //check that the file exists 
    let version_file = entry.path().join("version.txt");
    if version_file.exists() {
        let contents = fs::read_to_string(&version_file.to_str().unwrap()).unwrap();
        match Some(contents) {
            contents => {
                let first_split: &Vec<&str> = &contents
                    .unwrap()
                    .split(",")
                    .collect();
                let second_split: &Vec<&str> = &first_split[1]
                    .split(":")
                    .collect();
                let version = second_split[1].to_string().trim();
                    
                Some(version.to_string())
            },
            None => None
        }
    } else {
        write_verbose(&format!("version file does not exist {:#?}", &version_file.to_str()), v);
        None
    }

}
fn search(v: &bool) {
    write_verbose("Beginning search... ", v);
    let mut candidates = Vec::new();
    let mut count: u32 = 0;
    for entry in WalkDir::new("/") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            count += 1;
            if count % 100000 == 0 {
                write_verbose(&format!(
                        "Processed {} file ({})", count, entry
                        .path()
                        .display()), v);
            }
            if entry.file_name().to_str() == Some("nios2_command_shell.sh") {
                write_verbose(&format!(
                        "Found candidate at: {}", entry
                        .path()
                        .display()), v);
                candidates.push(entry);
            }
        }
    }
    write_verbose(&format!("Found {} candidates.", candidates.len()), v);
    let mut installations = Vec::new();
    for candidate in candidates {
        write_verbose(&format!("Considering candidate {}", candidate.path().display()), v);
        //match parse_version(candidate) {
        match Some("10") {
            Some(version) => { 
                let mut hash = HashMap::new();
                hash.insert("version", version);
                hash.insert("path", candidate.file_name().to_str().unwrap());
                installations.push(hash);
            },
            None => write_verbose("Missing or invalid version file.", v)
        }
    }
    write_verbose("Saving results... ", v);
    //write_cache_file(installations);

}

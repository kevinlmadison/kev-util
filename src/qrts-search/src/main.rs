/// Stub for documentation comments
extern crate appdirs;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;

use clap::App;
use regex::Regex;
use std::fmt;
use std::fs;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let v = matches.is_present("verbose");
    write_verbose("Verbose output enabled.", &v);
    write_verbose(&format!("Cache Directory: {}", get_cache_dir()), &v);
    if matches.is_present("purge") {
        purge(&v);
    }
    if matches.is_present("search") {
        purge(&v);
        search(&v);
    }
    if matches.is_present("list") {
        print_list(load_cache_file(&v));
    }
    match matches.value_of("set-preference") {
        Some(pref) => setpref(pref.parse::<usize>().unwrap(), &v),
        None => (),
    }
    match matches.value_of("list-version") {
        Some(version) => print_list(search_version(version, &v)),
        None => (),
    }
    if matches.is_present("get") {
        get(String::from(""), &v);
    }
    match matches.value_of("get-version") {
        Some(version) => get(version.to_string(), &v),
        None => (),
    }
}

fn write_verbose(message: &str, v: &bool) {
    if *v {
        io::stderr().write(&format!("{}", message).as_bytes()).unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_verbose() {
        let test_string = String::from("/home/kelevra/.local/share/quartus-search");
        println!("{}", &test_string);
        assert_eq!(test_string, get_cache_dir());
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
        match fs::read_to_string(&version_file.to_str().unwrap()) 
        {
            Ok(contents) => {
                let first_split: &Vec<&str> = &contents
                    .split(",")
                    .collect();
                let second_split: &Vec<&str> = &first_split[1]
                    .split(":")
                    .collect();
                Some(second_split[1].to_string().trim().to_string())
            },
            Err(_) => None
        }
    } else {
        write_verbose(&format!(
                "version file does not exist {:#?}",
                &version_file.to_str()),
                v);
        None
    }

}

#[derive(Serialize, Deserialize, Default)]
struct Install {
    version: String,
    path: String,
    preference: bool,
}

impl fmt::Display for Install {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v: {}, path: {}", self.version, self.path)
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
        write_verbose(&format!(
                "Considering candidate {}",
                 candidate.path().display()),
                 v);
        match parse_version(&candidate, v) {
            Some(vers) => { 
                let install = Install {
                    version: vers,
                    path: String::from(candidate
                                       .file_name()
                                       .to_str()
                                       .unwrap()),
                    ..Default::default()
                };
                installations.push(install);
            },
            None => write_verbose("Missing or invalid version file.", v)
        }
    }
    write_verbose("Saving results... ", v);
    write_cache_file(&installations, v);

}

fn write_cache_file(contents: &Vec<Install>, v: &bool) {
    if !Path::new(&get_cache_dir()).is_dir() {
        write_verbose("Creating cache dir... ", v);
        fs::DirBuilder::new()
            .recursive(true)
            .create(Path::new(&get_cache_dir()))
            .unwrap()
    }
    write_verbose("Writing data to cache file...", v);
    if !Path::new(&get_cache_file()).exists() {
        let mut file = fs::File::create(&get_cache_file()).unwrap();
        write!(file, "{}", serde_json::to_string(&contents).unwrap());
    } else {
        let mut file = fs::File::open(&get_cache_file()).unwrap();
        write!(file, "{}", serde_json::to_string(&contents).unwrap());
    }
    write_verbose("Write completed.", v);
}


fn load_cache_file(v: &bool) -> Vec<Install> {
    write_verbose("Loading cache file", v);
    if !Path::new(&get_cache_file()).exists() {
        write_verbose("Cache file does not exist, beginning search... ", v);
        search(v);
    }
    let mut file = fs::File::open(&get_cache_file()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let cache_file: Vec<Install> = serde_json::from_str(&buf).unwrap();
    cache_file
}

fn print_list(list: Vec<Install>) {
    eprintln!("Number\tPref\tVersion\tPath");
    let mut i: u32 = 0;
    for item in list {
        let mut preferred = String::new();
        if item.preference {
            preferred = String::from("*");
        }
        println!("{}\t{}\t{}\t{}", i, preferred, item.version, item.path);
        i += 1;
    }
}

fn search_version(target: &str, v: &bool) -> Vec<Install> {
    let query = format!(".*{}.*",target);
    let re = Regex::new(&query).unwrap();
    write_verbose(&format!("Searching for install with version matching: {}", &query), v);
    let mut results = Vec::new();
    for row in load_cache_file(v) {
        if re.is_match(&row.version) {
            results.push(row);
        }
    }
    write_verbose(&format!("Search found {} results.", results.len()), v);
    results
}

fn setpref(n: usize, v: &bool) {
    write_verbose(&format!("Toggling preference for installation {}", n), v);
    let mut installations = load_cache_file(v);
    if installations[n].preference {
        write_verbose("Toggling from true to false.", v);
        installations[n].preference = false;
    } else {
        write_verbose("Toggling from false to true.", v);
    }
    write_cache_file(&installations, v);
}

fn get(q: String, v: &bool) {
    let query = format!(".*{}.*", q);
    let re = Regex::new(&query).unwrap();
    write_verbose(&format!("Getting install directory with version matching {}...",&query), v);
    let mut result: Option<Install> = None;
    let mut best: Vec<Install> = Vec::new();

    for row in load_cache_file(v) {

        if !re.is_match(&row.version) { continue; }

        best.push(row);

        if best.last().unwrap().preference  == true {
            write_verbose(&format!("Preferred install found: {:#}", best.last().unwrap()), v);
            result = best.pop();
            break;
        }
    }

    match result {
        None => {
            write_verbose("No preferred install, failing over to non-preferred.", v);
            result = best.pop();
        },
        _ => (),
    }

    match result {
        None => {
            eprintln!("FATAL: no installation directory found");
            eprintln!("matching version query \'{}\'", &query);
            std::process::exit(1);
        },
        Some(install) => {
            write_verbose(&format!("Selected result: {:#}", &install), v);
            println!("{:#}", &install.path);
            std::process::exit(0);
        },
    }
}

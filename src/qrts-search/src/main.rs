#[macro_use]
extern crate clap;
extern crate walkdir;

use clap::App;
use walkdir::WalkDir;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("verbose") {
        println!("Verbose output enabled.");
    }
}

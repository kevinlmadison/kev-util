extern crate clap;
use clap::App;

fn main() {
    App::new("Quartus Search")
        .version("0.1")
        .author("Kevin Madison <coolklm121@gmail.com>")
        .about("This script can be used to locate installations of Altera Quartus\n
        (specifically the NIOS tooling packages installed therein). Once located,\n
        it will cache the located instances so that they can be queried by other\n
        scripts. Note that the cache is actually stored in user_data_dir\n
        (~/.local/share on Linux) to prevent it from being inadvertently deleted by\n
        cache cleaning.")
        .arg(Arg::with_name("purge")
             .short("p")
             .long("purge")
             .help("Purge cached installation directories"))
        .get_matches();
}

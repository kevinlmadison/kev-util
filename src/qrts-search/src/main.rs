extern crate clap;
extern crate walkdir;
use clap::{Arg, App, SubCommand};


fn main() {
    let matches = App::new("Quartus Search")
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
                          .arg(Arg::with_name("search")
                              .short("s")
                              .long("search")
                              .help("Force a fresh search for Quartus installation\n
                                      directories. This implies --purge. Note that the search \n
                                      operation can take a long time, as it traverses the entire\n
                                      filesystem in its search."))
                          .arg(Arg::with_name("list")
                              .short("l")
                              .long("list")
                              .help("List installation directories. If the cache is empty,\n
                                    then --search is implied"))
                          .arg(Arg::with_name("list_version")
                              .short("L")
                              .long("list_version")
                              .help("List installation directories where the detected version\n
                                    number is similar to the one specified here."))
                          .arg(Arg::with_name("set_pref")
                               .short("P"))
                          .arg(Arg::with_name("get")
                               .short("g"))
                          .arg(Arg::with_name("get_version")
                               .short("G"))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();
}

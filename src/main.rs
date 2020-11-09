use antsnap::config::*;
use antsnap::algo::*;

fn main() {
/*
    let matches = App::new("informer")
        .version("0.1")
        .author("William Gasper <wkg@williamgasper.com>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                // TODO: FIX THIS
                .default_value("")
                .long_help("Path to a TOML config file"),
        )
        .get_matches();
*/
    let cfg: Config = get_default_config();

    //if matches.is_present("config") {
    //    let cfg_path_str: &str = matches.value_of("config").unwrap();
    //    let cfg_path = PathBuf::from(cfg_path_str);
    //    cfg = load_config(&cfg_path);
    //}

    aco(&cfg);
}

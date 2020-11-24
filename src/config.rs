use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml::from_str;

// TODO: bools should probably be Strings that are then parsed to allow for
// fuzzy language usage
#[derive(Deserialize)]
pub struct AlgoParams {
    pub init_pheromone_val: Option<f64>,
    pub t_0: Option<f64>,
    pub evap_coeff: Option<f64>,
    pub epis_dim: Option<usize>,
    pub num_ants: Option<usize>,
    pub max_iters: Option<usize>,
    pub lambda: Option<f64>,
    pub data_fp: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub algo: AlgoParams,
}

pub fn load_config(file_path: &PathBuf) -> Config {
    let mut file = File::open(file_path.as_path()).unwrap_or_else(|why| {
        panic!(
            "Could not open config file: {}, why: {}",
            file_path.to_str().unwrap(),
            why
        );
    });
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap_or_else(|why| {
        panic!(
            "Could not read config file: {}, why: {}",
            file_path.to_str().unwrap(),
            why
        );
    });

    toml::from_str(contents.as_str()).unwrap()
}

pub fn get_default_config() -> Config {
    let config: Config = toml::from_str(
        r#"
    [algo]
    init_pheromone_val = 1.0
    t_0 = 0.8
    evap_coeff = 0.1
    epis_dim = 3
    num_ants = 1000
    max_iters = 40
    lambda = 2.0
    data_fp = 'data/gametes_small'
    "#,
    )
    .unwrap();

    config
}

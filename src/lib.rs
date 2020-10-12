pub mod config;
pub mod algo;

#[cfg(test)]
mod tests {
    use crate::config::*;

    #[test]
    fn get_default_config_0() {
        let cfg = get_default_config();

        println!("{:?}", cfg.algo.init_pheromone_val);
        let evap_coeff_expected: f64 = 0.8;
        assert_eq!(cfg.algo.evap_coeff, Some(evap_coeff_expected));
    }
}

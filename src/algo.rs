use rayon::prelude::*;

use crate::utils::*;
use crate::config::*;

pub fn aco(params: &Config) {
    //////
    let num_snps = 20;
    //////
    let mut num_ants = 2000;
    
    if let Some(k) = &params.algo.num_ants {
        num_ants = k.to_owned();
    }
    
    let mut epis_dim = 3;

    if let Some(dim) = &params.algo.epis_dim {
        epis_dim = dim.to_owned();
    }
    
    let mut threshold: f64 = 0.8;

    if let Some(t_0) = &params.algo.t_0 {
        threshold = t_0.to_owned();
    }
    
    let num_iters: usize = 100;
    
    // init pheromones matrix
    let mut pheromones = init_pheromones(num_snps);
    
    // give each ant its first snp
    let mut paths = init_ants(num_ants, num_snps, epis_dim);

    for _ in (0..num_iters) {
        // give each ant its first snp
        let mut paths = init_ants(num_ants, num_snps, epis_dim);
        

        // for each ant
        paths.par_iter_mut().map(|p| {
            expand_path(p, &pheromones, epis_dim, threshold);
        });

        // evaluate solutions
        //
        // update pheromones
        
        // use .par_iter() for rayon
            // select SNPs until desired dim

    }
}

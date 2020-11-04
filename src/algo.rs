use rayon::prelude::*;

use crate::config::*;
use crate::utils::*;

use logregressor::model::*;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

const LR_N_ITERS: usize = 500;
const LR_LEARN_RATE: f64 = 0.1;
const PROPORTION_TO_SELECT: f64 = 0.15;

pub fn aco(params: &Config) {
    let (x, y) = load_data(&params.algo.data_fp);

    let num_snps = x.1;
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

        let mut losses: Vec<(SNP, f64)> = Vec::new();

        // evaluate solutions
        for (idx, path) in paths.iter().enumerate() {
            let subset: Matrix = column_subset(&x, &path);
            
            let mut model = LogRegressor::new();
            model.train(&subset, &y, LR_N_ITERS, LR_LEARN_RATE);
            
            losses.push((idx.to_owned(), model.loss(&subset, &y)));
        }
        
        // sort losses
        losses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // select top proportion of solutions
        let partition: usize = losses.len() * PROPORTION_TO_SELECT as usize;

        // update pheromones
        for idx in 0..partition {
            let this_path = paths.get(losses.get(idx).unwrap().0).unwrap();


        }

        // select SNPs until desired dim
    }
}

// 10.1159/000085222
//
//

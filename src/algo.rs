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
    /// TODO move all this????
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

    let mut evap_coeff: f64 = 0.1;
    if let Some(evap) = &params.algo.evap_coeff {
        evap_coeff = evap.to_owned();
    }

    let mut lambda: f64 = 2.0;
    if let Some(lambda_in) = &params.algo.lambda {
        lambda = lambda_in.to_owned();
    }

    let num_iters: usize = 100;

    ///////////////
    // init pheromones matrix
    let mut pheromones: Matrix = init_pheromones(num_snps);

    // give each ant its first snp
    let mut paths: Vec<Vec<SNP>> = init_ants(num_ants, num_snps, epis_dim);

    for _ in (0..num_iters) {
        // give each ant its first snp
        let mut paths = init_ants(num_ants, num_snps, epis_dim);

        //for path in paths.iter_mut() {
        //    expand_path(path, &pheromones, epis_dim, threshold);
        //}
        // for each ant
        paths.par_iter_mut().map(|p| {
            expand_path(p, &pheromones, epis_dim, threshold);
        });

        let mut losses: Vec<(SNP, f64)> = Vec::new();

        let mut path_indices: Vec<usize> = Vec::new();
        for idx in 0..paths.len() {
            path_indices.push(idx);
        }

        // TODO HOW TO MAKE THSI WORK
        // map this to a fn that returns the loss value
        // then collect as losses
        //
        //let path_indices = (0..paths.len()).iter().collect();
        path_indices.par_iter().map(|idx| {
            // evaluate solutions
            //for (idx, path) in paths.iter().enumerate() {
            //println!("{:?}", path);
            let path = paths.get(idx.to_owned()).unwrap();
            // lol this does not appear to be working
            let subset: Matrix = column_subset(&x, &path);

            let mut model = LogRegressor::new();
            let loss = model.train(&subset, &y, LR_N_ITERS, LR_LEARN_RATE);

            losses.push((idx.to_owned(), loss));
        });

        // sort losses
        losses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // select top proportion of solutions
        let partition: usize = losses.len() * PROPORTION_TO_SELECT as usize;

        // update pheromones
        for idx in 0..partition {
            let this_path = paths.get(losses.get(idx).unwrap().0).unwrap();
            update_pheromones(&mut pheromones, &this_path, &evap_coeff, &lambda, true);
        }

        for idx in partition..losses.len() {
            let this_path = paths.get(losses.get(idx).unwrap().0).unwrap();
            update_pheromones(&mut pheromones, &this_path, &evap_coeff, &lambda, false);
        }

        // select SNPs until desired dim
    }
}

// 10.1159/000085222
//
//

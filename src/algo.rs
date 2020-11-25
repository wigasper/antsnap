use rayon::prelude::*; // 1.5.0

use crate::config::*;
use crate::utils::*;

use logregressor::model::*;
//use logregressor::utils::print_matrix;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

const LR_N_ITERS: usize = 500;
const LR_LEARN_RATE: f64 = 0.1;
const PROPORTION_TO_SELECT: f64 = 0.15;

pub fn train_one(idx: &usize, paths: &Vec<Vec<SNP>>, x: &Matrix, y: &Matrix) -> (usize, f64) {
    let path = paths.get(idx.to_owned()).unwrap();
    let subset: Matrix = column_subset(&x, &path);

    let mut model = LogRegressor::new();
    let loss = model.train(&subset, &y, LR_N_ITERS, LR_LEARN_RATE);

    (idx.to_owned(), loss)
    //losses.push((idx.to_owned(), loss));
}

pub fn aco(params: &Config) {
    let (x, y, header): (Matrix, Matrix, Vec<String>) = load_data(&params.algo.data_fp);

    let num_snps = x.1;
    //////
    // TODO move all this????
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

    let num_iters: usize = 50;
    
    // retain the top solutions
    let mut top_solutions: Vec<(Vec<String>, f64)> = Vec::new();
    // top paths at any given iter
    //let mut top_three: Vec<(usize, f64)> = Vec::new();
    ///////////////
    // init pheromones matrix
    let mut pheromones: Matrix = init_pheromones(num_snps);

    for _ in 0..num_iters {
        // give each ant its first snp
        let mut paths: Vec<Vec<SNP>> = init_ants(num_ants, num_snps, epis_dim);

        //for path in paths.iter_mut() {
        //    expand_path(path, &pheromones, epis_dim, threshold);
        //}
        // for each ant
        //paths.par_iter_mut().map(|p| {
        //    expand_path(p, &pheromones, epis_dim, threshold);
        //});
        paths.par_iter_mut().for_each(|p| {
            expand_path(p, &pheromones, epis_dim, threshold);
        });

        let path_indices: Vec<usize> = (0..paths.len()).into_iter().collect();

        let par_iter = path_indices
            .par_iter()
            .map(|idx| train_one(idx, &paths, &x, &y));

        let mut losses: Vec<(usize, f64)> = par_iter.collect();

        // sort losses
        losses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        

        for idx in 0..3 {
            let path: Vec<SNP> = paths.get(losses.get(idx).unwrap().0).unwrap().to_owned();
            let snps: Vec<String> = path.iter().map(|s| header.get(s.to_owned()).unwrap().to_owned()).collect();
            top_solutions.push((snps, losses.get(idx).unwrap().1));
        }

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
        /*
        if iter == num_iters - 1 {
            for index in (0..3) {
                let path: &Vec<SNP> = paths.get(losses.get(index).unwrap().0).unwrap();
                let snps: Vec<String> = path.iter().map(|s| header.get(s.to_owned()).unwrap().to_owned()).collect();
                println!("Path: {:?}", snps);
            }
        }*/

    }

    top_solutions.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for idx in 0..10 {
        let sol: &(Vec<String>, f64) = top_solutions.get(idx).unwrap();
        println!("Path: {:?} Loss: {}", sol.0, sol.1);
    }

    //print_matrix(&pheromones);
}

// 10.1159/000085222
//
//

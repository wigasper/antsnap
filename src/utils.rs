
use crate::config::*;

use rand::prelude::*;
use rand::seq::SliceRandom;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

const FP_EQUALITY_THRESH: f64 = 0.001;

pub fn get_r(i: &SNP, j: &SNP, pheromones: &Matrix, 
             current_path: &Vec<SNP>) -> f64 {
    
    // TODO: if things are slow this memory allocation could easily be
    // removed. leaving for readability now
    let unvisited_neighbors: Vec<SNP> = (0..pheromones.1).filter(|n| {
        !current_path.contains(n) && n != i
    }).collect();

    let row_start_idx: usize = i * pheromones.1;
    
    let mut rolling_sum: f64 = 0.0;

    for neigh in unvisited_neighbors.iter() {
        rolling_sum += pheromones.0.get(row_start_idx + neigh).unwrap();
    }

    let tau_ij: &f64 = pheromones.0.get(row_start_idx + j).unwrap();

    tau_ij / rolling_sum
}

pub fn transfer_prob(i: &SNP, j: &SNP, pheromones: &Matrix,
                     current_path: &Vec<SNP>,
                   rng: &mut ThreadRng, threshold: f64) -> f64 {
    let mut prob_out: f64 = 0.0;

    let q: f64 = rng.gen();

    if q > threshold {
        prob_out = 1.0
    } else {
        prob_out = get_r(i, j, pheromones, current_path);
    }

    prob_out
}

pub fn get_max(slice: &[f64]) -> f64 {
    let mut max: &f64 = slice.get(0).unwrap();
    
    for val in slice.iter() {
        if val > max {
            max = val;
        }
    }

    max.to_owned()
}

pub fn expand_path(current_path: &mut Vec<SNP>, pheromones: &Matrix,
                   epis_dim: usize, threshold: f64) { 
                   //params: &Config) {
    let mut rng = rand::thread_rng();

    while current_path.len() < epis_dim {
        add_to_path(current_path, pheromones, threshold, &mut rng);
    }
}

// add the next SNP to the path
pub fn add_to_path(current_path: &mut Vec<SNP>, //snps: &Vec<SNP>, 
                   pheromones: &Matrix, 
                   threshold: f64,
                   rng: &mut ThreadRng) {

    let i: &SNP = current_path.last().unwrap();
    
    let mut probs: Vec<f64> = Vec::new();

    for snp in (0..pheromones.1) {
        if !current_path.contains(&snp) {
            probs.push(transfer_prob(i, &snp, pheromones, current_path, rng, threshold));
        }
    }


    //////////////////
    let unvisited_neighbors: Vec<SNP> = (0..pheromones.1).filter(|snp| {
        !current_path.contains(snp) && snp != i
    }).collect();

    // wtf
    //let probs: Vec<f64> = unvisited_neighbors.iter().map(|j| {
    //    transfer_prob(i, j, pheromones, current_path, rng, threshold);
    //}).collect();

    //let mut probs: Vec<f64> = Vec::new();

    //for neighbor in unvisited_neighbors.iter() {
    //    probs.push(transfer_prob(i, neighbor, pheromones, current_path, rng, threshold));
   // }

    let max_prob = get_max(&probs);
    
    let mut snps_at_max: Vec<&SNP> = Vec::new();

    for (idx, prob) in probs.iter().enumerate() {
        if (prob - max_prob).abs() < FP_EQUALITY_THRESH {
            snps_at_max.push(unvisited_neighbors.get(idx).unwrap());
        }
    }
    
    current_path.push(snps_at_max.choose(rng).unwrap().to_owned().to_owned());
    /*
    let mut putative_snp: SNP = snps.choose(mut rng);

    while current_path.contains(putative_snp) {
        putative_snp = snps.choose(mut rng);
    }

    let q: f64 = rng.gen();
    
    let mut threshold: f64 = 0.8;

    if let Some(t_0) = &params.algo.t_0 {
        threshold = t_0;
    }

    if q > threshold {
        current_path.push(putative_snp);
    } else {
        
    }*/
}

pub fn init(num_ants: usize, num_snps: usize, 
            epis_dim: usize) -> (Matrix, Vec<Vec<SNP>>) {
    let mut matrix_out: Matrix = (Vec::new(), num_snps);
    let mut paths_out: Vec<Vec<SNP>> = Vec::with_capacity(epis_dim);

    let mut rng = rand::thread_rng();

    for _ in (0..num_snps) {
        for _ in (0..num_snps) {
            matrix_out.0.push(1.0);
        }
    }
    
    for _ in (0..num_ants) {
        let num: usize = rng.gen_range(0, num_snps);
        let path = vec![num];
        paths_out.push(path);
    }


    (matrix_out, paths_out)
}

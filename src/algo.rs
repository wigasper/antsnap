use rayon::prelude::*; // 1.5.0

use crate::config::*;
use crate::utils::*;

use logregressor::model::*;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

// logregressor params
const LR_N_ITERS: usize = 500;
const LR_LEARN_RATE: f64 = 0.1;

// top proportion of solutions to boost pheromone vals for
const PROPORTION_TO_SELECT: f64 = 0.05;
// number of solutions to retain per round
const N_SOLUTIONS_TO_RETAIN: usize = 10;

// calculates the Chi-square test stat for a contingency table
pub fn chi_square_test(contingency_table: &Matrix) -> f64 {
    let expected_freqs: Matrix = get_expected_freqs(contingency_table);

    let mut chi_square = 0.0;

    for (idx, observed) in contingency_table.0.iter().enumerate() {
        let expected = expected_freqs.0.get(idx).unwrap();
        if expected != &0.0 {
            chi_square += (observed - expected).powi(2) / expected;
        }
    }

    chi_square
}

// Function used to calculate the Chi square test stat for a single path
pub fn train_one_x2(idx: &usize, paths: &Vec<Vec<SNP>>, x: &Matrix, y: &Matrix) -> (usize, f64) {
    let path = paths.get(idx.to_owned()).unwrap();
    let subset: Matrix = column_subset(&x, &path);
    let contingency_table: Matrix = build_contingency_table(&subset, &y);
    let test_stat: f64 = chi_square_test(&contingency_table);

    (idx.to_owned(), test_stat)
}

// Trains a logistic regression model for a single path
pub fn train_one(idx: &usize, paths: &Vec<Vec<SNP>>, x: &Matrix, y: &Matrix) -> (usize, f64) {
    let path = paths.get(idx.to_owned()).unwrap();
    let mut subset: Matrix = column_subset(&x, &path);

    let int_term: Matrix = get_interactive_term(&subset);

    subset = append_columns(&subset, &int_term);

    let mut model = LogRegressor::new();
    let loss = model.train(&subset, &y, LR_N_ITERS, LR_LEARN_RATE);

    (idx.to_owned(), loss)
}

// Ant colony optimization algorithm routine
pub fn aco(params: &Config) {
    // load data
    let (x, y, header): (Matrix, Matrix, Vec<String>) = load_data(&params.algo.data_fp);

    let num_snps = x.1;
    //////
    // Load parameters
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

    let mut num_iters: usize = 50;
    if let Some(num_iters_in) = &params.algo.max_iters {
        num_iters = num_iters_in.to_owned();
    }

    // retain the top solutions
    // NOTE: this vec is named with the intention of using logistic regression
    // as the objective, but is confusing when using Chi square test
    let mut top_losses: Vec<(Vec<String>, Vec<SNP>, f64)> = Vec::new();

    // init pheromones matrix
    let mut pheromones: Matrix = init_pheromones(num_snps);

    for _ in 0..num_iters {
        // give each ant its first snp
        let mut paths: Vec<Vec<SNP>> = init_ants(num_ants, num_snps, epis_dim);

        paths.par_iter_mut().for_each(|p| {
            expand_path(p, &pheromones, epis_dim, threshold);
        });

        // A vec of indices for the paths, used later for mapping losses,
        // could be done without this though if par_iter guarantees
        // order (I think it does?)
        let path_indices: Vec<usize> = (0..paths.len()).into_iter().collect();

        let par_iter = path_indices
            .par_iter()
            .map(|idx| train_one_x2(idx, &paths, &x, &y));

        let mut losses: Vec<(usize, f64)> = par_iter.collect();

        // sort losses
        losses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for idx in 0..N_SOLUTIONS_TO_RETAIN {
            let path: Vec<SNP> = paths.get(losses.get(idx).unwrap().0).unwrap().to_owned();
            let snps: Vec<String> = path
                .iter()
                .map(|s| header.get(s.to_owned()).unwrap().to_owned())
                .collect();
            top_losses.push((snps, path, losses.get(idx).unwrap().1));
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
    }

    top_losses.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    // get the top Chi square test stats, this is unnecessary though
    // if using Chi square test as the objective function
    let mut top_chi_stats: Vec<(Vec<String>, f64)> = Vec::new();

    for solution in top_losses.iter() {
        let sol: &(Vec<String>, Vec<SNP>, f64) = solution;
        let col_subset: Matrix = column_subset(&x, &sol.1);
        let contingency_table: Matrix = build_contingency_table(&col_subset, &y);
        let test_stat: f64 = chi_square_test(&contingency_table);

        top_chi_stats.push((sol.0.to_owned(), test_stat));
    }

    top_chi_stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nCHI SQUARE TEST STATS");
    for idx in 0..30 {
        let this_sol = top_chi_stats.get(idx).unwrap();
        println!("Path: {:?}\tX2 test stat: {}", this_sol.0, this_sol.1);
    }

    // Check to see what the Chi square test stat is of the true
    // solution (make sure GAMETES modeling is working correctly
    let true_sol: Vec<SNP> = vec![x.1 - 3, x.1 - 2, x.1 - 1];
    let mut col_subset: Matrix = column_subset(&x, &true_sol);
    let contingency_table: Matrix = build_contingency_table(&col_subset, &y);
    let test_stat: f64 = chi_square_test(&contingency_table);
    println!("True sol test stat: {}", test_stat);

//    let int_term: Matrix = get_interactive_term(&col_subset);

//    col_subset = append_columns(&col_subset, &int_term);

//    let mut model = LogRegressor::new();
//    let loss = model.train(&col_subset, &y, LR_N_ITERS, LR_LEARN_RATE);
//    println!("True sol loss: {}", loss);
}

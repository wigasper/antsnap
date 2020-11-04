use crate::config::*;

use rand::prelude::*;
use rand::seq::SliceRandom;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

const FP_EQUALITY_THRESH: f64 = 0.001;

pub fn get_r(i: &SNP, j: &SNP, pheromones: &Matrix, current_path: &Vec<SNP>) -> f64 {
    // TODO: if things are slow this memory allocation could easily be
    // removed. leaving for readability now
    let unvisited_neighbors: Vec<SNP> = (0..pheromones.1)
        .filter(|n| !current_path.contains(n) && n != i)
        .collect();

    let row_start_idx: usize = i * pheromones.1;

    let mut rolling_sum: f64 = 0.0;

    for neigh in unvisited_neighbors.iter() {
        rolling_sum += pheromones.0.get(row_start_idx + neigh).unwrap();
    }

    let tau_ij: &f64 = pheromones.0.get(row_start_idx + j).unwrap();

    tau_ij / rolling_sum
}

pub fn transfer_prob(
    i: &SNP,
    j: &SNP,
    pheromones: &Matrix,
    current_path: &Vec<SNP>,
    rng: &mut ThreadRng,
    threshold: f64,
) -> f64 {
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

    // TODO: there is an early exit here, might see if this
    // actually is beneficial
    for val in slice.iter() {
        if (max - 1.0).abs() < FP_EQUALITY_THRESH {
            break;
        } else if val > max {
            max = val;
        }
    }

    max.to_owned()
}

pub fn expand_path(
    current_path: &mut Vec<SNP>,
    pheromones: &Matrix,
    epis_dim: usize,
    threshold: f64,
) {
    //params: &Config) {
    let mut rng = rand::thread_rng();

    while current_path.len() < epis_dim {
        add_to_path(current_path, pheromones, threshold, &mut rng);
    }
}

// add the next SNP to the path
pub fn add_to_path(
    current_path: &mut Vec<SNP>,
    pheromones: &Matrix,
    threshold: f64,
    rng: &mut ThreadRng,
) {
    let i: &SNP = current_path.last().unwrap();

    let mut probs: Vec<f64> = Vec::new();

    for snp in (0..pheromones.1) {
        if !current_path.contains(&snp) {
            probs.push(transfer_prob(
                i,
                &snp,
                pheromones,
                current_path,
                rng,
                threshold,
            ));
        }
    }

    let max_prob = get_max(&probs);

    let mut snps_at_max: Vec<SNP> = Vec::new();

    for (idx, prob) in probs.iter().enumerate() {
        if (prob - max_prob).abs() < FP_EQUALITY_THRESH {
            snps_at_max.push(idx);
        }
    }

    current_path.push(snps_at_max.choose(rng).unwrap().to_owned());
}

pub fn init_ants(num_ants: usize, num_snps: usize, epis_dim: usize) -> Vec<Vec<SNP>> {
    let mut paths_out: Vec<Vec<SNP>> = Vec::with_capacity(epis_dim);

    let mut rng = rand::thread_rng();

    for _ in (0..num_ants) {
        let num: usize = rng.gen_range(0, num_snps);
        let path = vec![num];
        paths_out.push(path);
    }

    paths_out
}

pub fn init_pheromones(num_snps: usize) -> Matrix {
    let mut matrix_out: Matrix = (Vec::new(), num_snps);

    for _ in (0..num_snps) {
        for _ in (0..num_snps) {
            matrix_out.0.push(1.0);
        }
    }

    matrix_out
}

//type Matrix = (Vec<Element>, usize);
// returns a m x 1 matrix
pub fn get_column(m: &Matrix, j: usize) -> Matrix {
    let mut m_out: Matrix = (Vec::new(), 1);

    let n_rows: usize = m.0.len() / m.1;

    for row in 0..n_rows {
        m_out.0.push(m.0.get(row * m.1 + j).unwrap().to_owned());
    }

    m_out
}

// append b columns to a
pub fn append_columns(a: &Matrix, b: &Matrix) -> Matrix {
    let a_n_rows: usize = a.0.len() / a.1;
    let b_n_rows: usize = b.0.len() / b.1;

    if a_n_rows != b_n_rows {
        panic!("utils::append_columns - matrices do not have same number of rows!");
    }

    let mut m_out: Matrix = (Vec::new(), a.1 + b.1);

    for row in 0..a_n_rows {
        let a_start_idx: usize = row * a.1;
        for a_idx in a_start_idx..(row * a.1 + a.1) {
            m_out.0.push(a.0.get(a_idx).unwrap().to_owned());
        }

        let b_start_idx: usize = row * b.1;
        for b_idx in b_start_idx..(row * b.1 + b.1) {
            m_out.0.push(b.0.get(b_idx).unwrap().to_owned());
        }
    }

    m_out
}

pub fn transpose(m: &Matrix) -> Matrix {
    let mut m_out: Matrix = (Vec::new(), m.0.len() / m.1);

    for col in 0..m.1 {
        for row in 0..(m.0.len() / m.1) {
            m_out.0.push(m.0.get(row * m.1 + col).unwrap().to_owned());
        }
    }

    m_out
}

pub fn append_rows(a: &mut Matrix, b: &Matrix) {
    if a.1 != b.1 {
        panic!("utils::append_rows - matrices do not have same dims");
    }

    for val in b.0.iter() {
        a.0.push(val.to_owned());
    }
}

// TODO: better way to do this, probably:
//      each needed column is added in a row, this si super easy just w/ vals
//      then transpose at the end
pub fn column_subset(m: &Matrix, cols: &Vec<usize>) -> Matrix {
    let init_vals: Vec<Element> = get_column(m, cols.first().unwrap().to_owned()).0;
    let mut m_out_T: Matrix = (init_vals, m.0.len() / m.1);

    for idx in 1..cols.len() {
        let mut this_col = get_column(m, cols.get(idx).unwrap().to_owned());
        // modify dim, transposing the col:
        this_col.1 = this_col.0.len();
        append_rows(&mut m_out_T, &this_col);
    }

    transpose(&m_out_T)
}

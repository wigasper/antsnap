use std::fs::File;
use std::io::prelude::*;

use rand::prelude::*;
use rand::seq::SliceRandom;

type SNP = usize;
type Element = f64;
type Matrix = (Vec<Element>, usize);

// threshold for comparing f64s
const FP_EQUALITY_THRESH: f64 = 0.001;

// Get r for a given SNP pair
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

// get a transfer probability for moving from SNP i to SNP j
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
        prob_out = 1.0;
    } else {
        prob_out = get_r(i, j, pheromones, current_path);
    }

    prob_out
}

// get the max from a slice of f64
pub fn get_max(slice: &[f64]) -> f64 {
    let mut max: &f64 = slice.get(0).unwrap();

    for val in slice.iter() {
        // max prob is 1.0, so no use in continuing if it's there
        if (max - 1.0).abs() < FP_EQUALITY_THRESH {
            break;
        } else if val > max {
            max = val;
        }
    }

    max.to_owned()
}

// expands a single path until the desired dimension
pub fn expand_path(
    current_path: &mut Vec<SNP>,
    pheromones: &Matrix,
    epis_dim: usize,
    threshold: f64,
) {
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

    // get the probability of moving to all other SNPs
    let mut probs: (Vec<usize>, Vec<f64>) = (Vec::new(), Vec::new());

    for snp in 0..pheromones.1 {
        if !current_path.contains(&snp) {
            probs.1.push(transfer_prob(
                i,
                &snp,
                pheromones,
                current_path,
                rng,
                threshold,
            ));
            probs.0.push(snp);
        }
    }

    // get all SNPs at max prob.
    let max_prob = get_max(&probs.1);

    let mut snps_at_max: Vec<SNP> = Vec::new();

    for (idx, prob) in probs.1.iter().enumerate() {
        if (prob - max_prob).abs() < FP_EQUALITY_THRESH {
            snps_at_max.push(probs.0.get(idx).unwrap().to_owned());
        }
    }

    // select a random SNP from all the SNPs at max probability
    current_path.push(snps_at_max.choose(rng).unwrap().to_owned());
}

// initialize ants with a random SNP
pub fn init_ants(num_ants: usize, num_snps: usize, epis_dim: usize) -> Vec<Vec<SNP>> {
    let mut paths_out: Vec<Vec<SNP>> = Vec::with_capacity(epis_dim);

    let mut rng = rand::thread_rng();

    for _ in 0..num_ants {
        let num: usize = rng.gen_range(0, num_snps);
        let path = vec![num];
        paths_out.push(path);
    }

    paths_out
}

// initialize all pheromones as 1. this could be changed in the
// future
pub fn init_pheromones(num_snps: usize) -> Matrix {
    let mut matrix_out: Matrix = (Vec::new(), num_snps);

    for _ in 0..num_snps {
        for _ in 0..num_snps {
            matrix_out.0.push(1.0);
        }
    }

    matrix_out
}

// returns a m x 1 matrix for column j
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

// transposes a matrix
pub fn transpose(m: &Matrix) -> Matrix {
    let mut m_out: Matrix = (Vec::new(), m.0.len() / m.1);

    for col in 0..m.1 {
        for row in 0..(m.0.len() / m.1) {
            m_out.0.push(m.0.get(row * m.1 + col).unwrap().to_owned());
        }
    }

    m_out
}

// appends the rows in b to a
pub fn append_rows(a: &mut Matrix, b: &Matrix) {
    if a.1 != b.1 {
        panic!("utils::append_rows - matrices do not have same dims");
    }

    for val in b.0.iter() {
        a.0.push(val.to_owned());
    }
}

// returns a matrix that is a subset of the columns in m,
// columns are designated by the indices in the cols Vec
pub fn column_subset(m: &Matrix, cols: &Vec<usize>) -> Matrix {
    let init_vals: Vec<Element> = get_column(m, cols.first().unwrap().to_owned()).0;
    let mut m_out_t: Matrix = (init_vals, m.0.len() / m.1);

    for idx in 1..cols.len() {
        let mut this_col = get_column(m, cols.get(idx).unwrap().to_owned());
        // modify dim, transposing the col:
        this_col.1 = this_col.0.len();
        append_rows(&mut m_out_t, &this_col);
    }

    transpose(&m_out_t)
}

// loads a dataset formatted like GAMETES 2.0 output
// final vec<string> in tuple is the header key
pub fn load_data(fp: &String) -> (Matrix, Matrix, Vec<String>) {
    let mut file = File::open(fp).unwrap();
    let mut str_in = String::new();

    file.read_to_string(&mut str_in).unwrap_or_else(|why| {
        panic!("Could not read data in: {}", why);
    });

    let mut x: Matrix = (Vec::new(), 0);
    let mut y: Matrix = (Vec::new(), 1);
    let mut header: Vec<String> = Vec::new();

    for line in str_in.split("\n") {
        if line.len() > 0 {
            if line.starts_with("N") {
                header = line.split_whitespace().map(|s| s.to_owned()).collect();
            } else {
                let mut vals: Vec<&str> = line.split_whitespace().collect();

                // check dim
                if x.1 == 0 {
                    x.1 = vals.len() - 1;
                }

                let y_val = vals.pop().unwrap();

                y.0.push(y_val.parse::<f64>().unwrap_or_else(|why| {
                    panic!("Could not parse {} to f64: {}", y_val, why);
                }));

                for val in vals.iter() {
                    x.0.push(val.parse::<f64>().unwrap_or_else(|why| {
                        panic!("Could not parse {} to f64: {}", val, why);
                    }));
                }
            }
        }
    }

    (x, y, header)
}

// update the pheromone value for a single pheromone
pub fn update_single_pheromone(
    pheromones: &mut Matrix,
    idx: usize,
    evap_coeff: &f64,
    lambda: &f64,
    good_solution: bool,
) {
    if let Some(val) = pheromones.0.get_mut(idx) {
        if good_solution {
            *val = (1.0 - evap_coeff) * *val + evap_coeff * lambda;
        } else {
            *val = (1.0 - evap_coeff) * *val;
        }
    }
}

// update the pheromone values for a single path
pub fn update_pheromones(
    pheromones: &mut Matrix,
    path: &Vec<SNP>,
    evap_coeff: &f64,
    lambda: &f64,
    good_solution: bool,
) {
    for source_idx in 0..path.len() - 1 {
        let source: SNP = path.get(source_idx).unwrap().to_owned();
        let sink: SNP = path.get(source_idx + 1).unwrap().to_owned();

        let pher_idx_0: usize = source * pheromones.1 + sink;
        let pher_idx_1: usize = sink * pheromones.1 + source;

        update_single_pheromone(pheromones, pher_idx_0, evap_coeff, lambda, good_solution);
        update_single_pheromone(pheromones, pher_idx_1, evap_coeff, lambda, good_solution);
    }
}

// a one-hot encoding function that was tested with the logistic regression
// objective. probably only need two variables for each?
//
// leaving for now, NOTE unused
//
// not a general function. slapped together only for use with this
// data
pub fn naive_one_hot(x: &Matrix) -> Matrix {
    let mut m_out: Matrix = (Vec::new(), 0);

    for col_idx in 0..x.1 {
        let mut new_cols: Matrix = (Vec::with_capacity((x.0.len() / x.1) * 3), 3);

        for row_idx in 0..(x.0.len() / x.1) {
            let element_idx: usize = col_idx * x.1 + row_idx;
            let element: &Element = x.0.get(element_idx).unwrap();

            if element == &0.0 {
                new_cols.0.push(1.0);
                new_cols.0.push(0.0);
                new_cols.0.push(0.0);
            } else if element == &1.0 {
                new_cols.0.push(0.0);
                new_cols.0.push(1.0);
                new_cols.0.push(0.0);
            } else {
                new_cols.0.push(0.0);
                new_cols.0.push(0.0);
                new_cols.0.push(1.0);
            }
        }

        if col_idx == 0 {
            m_out = new_cols;
        } else {
            m_out = append_columns(&m_out, &new_cols);
        }
    }

    m_out
}

// get the interactive term value for a given x matrix
pub fn get_interactive_term(x: &Matrix) -> Matrix {
    let mut m_out: Matrix = (Vec::new(), 1);

    for row_idx in 0..(x.0.len() / x.1) {
        let row_start_idx = row_idx * x.1;
        let row_end_idx = row_start_idx + x.1;

        let mut product: f64 = 1.0;

        for idx in row_start_idx..row_end_idx {
            product = product * x.0.get(idx).unwrap();
        }

        m_out.0.push(product);
    }

    m_out
}

// build a contingency table for Chi square test
// NOTE this works only for 3-snp combos
pub fn build_contingency_table(x: &Matrix, y: &Matrix) -> Matrix {
    let mut contingency_table: Matrix = (
        Vec::with_capacity(3usize.pow(x.1 as u32) * 2),
        3usize.pow(x.1 as u32),
    );

    for _ in 0..(3usize.pow(x.1 as u32) * 2) {
        contingency_table.0.push(0.0);
    }

    let n_rows = x.0.len() / x.1;

    for row_idx in 0..n_rows {
        let mut x_vals: Vec<f64> = Vec::with_capacity(x.1);
        for col_idx in 0..x.1 {
            let idx = x.1 * row_idx + col_idx;
            x_vals.push(x.0.get(idx).unwrap().to_owned());
        }

        let mut table_idx: usize = (x_vals.get(0).unwrap() * 9.0
            + x_vals.get(1).unwrap() * 3.0
            + x_vals.get(2).unwrap()) as usize;

        if y.0.get(row_idx).unwrap() == &1.0 {
            table_idx += 3usize.pow(x.1 as u32);
        }

        if let Some(val) = contingency_table.0.get_mut(table_idx) {
            *val += 1.0;
        }
    }

    contingency_table
}

// Get the sum of a column
pub fn col_sum(m: &Matrix, col: usize) -> f64 {
    let mut sum: f64 = 0.0;

    for row_idx in 0..(m.0.len() / m.1) {
        sum += m.0.get(m.1 * row_idx + col).unwrap();
    }

    sum
}

// get the sum of a row
pub fn row_sum(m: &Matrix, row: usize) -> f64 {
    let mut sum: f64 = 0.0;

    for col_idx in 0..m.1 {
        sum += m.0.get(m.1 * row + col_idx).unwrap();
    }

    sum
}

// get the expected frequency table for Chi square test
pub fn get_expected_freqs(table: &Matrix) -> Matrix {
    let total: f64 = table.0.iter().sum();
    let mut table_out: Matrix = (Vec::with_capacity(table.0.len()), table.1);

    // this is the slow way
    let n_rows = table.0.len() / table.1;

    for row_idx in 0..n_rows {
        for col_idx in 0..table.1 {
            table_out
                .0
                .push((col_sum(table, col_idx) * row_sum(table, row_idx)) / total);
        }
    }

    table_out
}

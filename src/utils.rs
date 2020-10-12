use rand::seq::SliceRandom;

type SNP = String;
type Element = f64;
type Matrix = (Vec<Element>, usize);

pub fn probability(i: &SNP, j: &SNP, pheromones: &Matrix,
                   rng: &mut ThreadRng, threshold: f64) -> f64 {
    let mut prob_out: f64 = 0.0;

    let q: f64 = rng.gen();

    if q > threshold {
        prob_out = 1.0
    } else {
        prob_out = get_R();
    }

    return prob_out;
}

// add the next SNP to the path
pub fn add_to_path(current_path: &Vec<String>, snps: &Vec<SNP>, 
                   pheromones: &Matrix, rng: &mut ThreadRng,
                   params: &Config) {
    
    let mut putative_snp: SNP = snps.choose(mut rng);

    while !current_path.contains(putative_snp) {
        putative_snp = snps.choose(mut rng);
    }

    let q: f64 = rng.gen();
    
    let mut threshold: f64 = 0.8;

    if let Some(t_0) = &params.algo.t_0 {
        threshold = t_0;
    }

    if q > threshold {
        
    }
    
}

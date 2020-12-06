pub mod algo;
pub mod config;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::algo::*;
    use crate::config::*;
    use crate::utils::*;

    type Element = f64;
    type Matrix = (Vec<Element>, usize);

    #[test]
    fn get_col_0() {
        let vals = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7];
        let m: Matrix = (vals, 3);

        let col = get_column(&m, 1);

        assert_eq!(vec![2.3, 5.6], col.0);
    }

    #[test]
    fn append_columns_0() {
        let a_vals = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7];
        let a: Matrix = (a_vals, 3);

        let b_vals = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7];
        let b: Matrix = (b_vals, 3);

        let result = append_columns(&a, &b);
        let e_vals = (vec![1.2, 2.3, 3.4, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 4.5, 5.6, 6.7]);
        let expected = (e_vals, 6);

        assert_eq!(result.0, expected.0);
        assert_eq!(result.1, expected.1);
    }

    #[test]
    fn transpose_0() {
        let vals = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7];
        let m: Matrix = (vals, 3);

        let result = transpose(&m);

        let e_vals = vec![1.2, 4.5, 2.3, 5.6, 3.4, 6.7];

        assert_eq!(result.1, 2);
        assert_eq!(result.0, e_vals);
    }

    #[test]
    fn get_default_config_0() {
        let cfg = get_default_config();

        let evap_coeff_expected: f64 = 0.1;
        assert_eq!(cfg.algo.evap_coeff, Some(evap_coeff_expected));
    }
    #[test]
    fn column_subset_0() {
        let a_vals = vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 1.3, 2.5, 5.8];
        let a: Matrix = (a_vals, 3);

        let cols = vec![0, 2];

        let b = column_subset(&a, &cols);

        let e_vals = vec![1.2, 3.4, 4.5, 6.7, 1.3, 5.8];
        let expected: Matrix = (e_vals, 2);

        assert_eq!(b, expected);
    }

    #[test]
    fn get_expected_freqs_0() {
        let t: Matrix = (vec![3.0, 2.0, 2.0, 4.0, 0.0, 1.0], 3);

        let actual: Matrix = get_expected_freqs(&t);
        let expect: Matrix = (
            vec![
                4.083333333333333,
                1.1666666666666667,
                1.75,
                2.9166666666666665,
                0.8333333333333334,
                1.25,
            ],
            3,
        );

        assert_eq!(actual, expect);
    }

    #[test]
    fn chi_square_test_0() {
        let t: Matrix = (vec![3.0, 2.0, 2.0, 4.0, 0.0, 1.0], 3);
        let actual: f64 = chi_square_test(&t);
        let expect: f64 = 2.204081632653061;

        assert_eq!(actual, expect);
    }
}

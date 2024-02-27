use ndarray::Array2;
//use ndarray_linalg::cholesky::*;
use crate::math::cholescky_factorization::cholesky_decomposition;
use crate::definitions::Real;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use log::info;

// make a 2-D array of shape (n, steps) of f32 (Real) random numbers
// This indicates a path of n objects with steps observations
// And the objects are correlated
// The correlation is given by the cholesky decomposition of the correlation matrix
pub fn correlated_path(steps: usize, correlation: &Array2<Real>) -> Array2<Real> {
    let n = correlation.shape()[0];
    let mut rng = thread_rng();
    let normal: Normal<Real> = Normal::new(0.0, 1.0).unwrap();
    let cholesky = cholesky_decomposition(correlation).unwrap();

    let mut msg = String::from("better to calculate cholesky decomposition once and use it for all paths.");
    msg.push_str( "In addition, check BLAS is being used. Currently disabled for multi developing environment");
    info!("{}", msg);
    cholesky.dot(&Array2::from_shape_fn((n, steps), |_| normal.sample(&mut rng) as Real))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    use ndarray::prelude::*;
    
    #[test]
    fn test_correlated_path() {
        let steps = 100000;
        let corr = 0.5 as Real;
        let cholesky_variable = (1.0 - corr.powi(2)).sqrt();

        let correlation_matrix = Array2::from_shape_vec(
            (2, 2),
            vec![1.0, 0.5, 0.5, 1.0],
        )
        .unwrap();
        let path = correlated_path(steps, &correlation_matrix);
        let mean = path.mean_axis(Axis(1)).unwrap();
        // discrepancy: subtract mean vector from all path columnwise
        let discrepancy = path.clone() - mean.clone().insert_axis(Axis(1));
        let recalc_cov = discrepancy.dot(&discrepancy.t()) / (steps as Real);

        let mut msg = String::from("Statistics:\n");
        msg.push_str(&format!(" * # of variables: {}\n", 2));
        msg.push_str(&format!(" * Steps: {}\n", steps));
        msg.push_str(&format!(" * sampled from a normal distribution with mean = {}, correlation = {}\n", 0.0, corr));
        msg.push_str(&format!(" * sampled mean: {}\n", mean));
        msg.push_str(&format!(" * sampled covariance: \n{}\n", &recalc_cov));

        println!("{}", msg);
        let cholesky = cholesky_decomposition(&correlation_matrix).unwrap();
        
        //let expected_cov = cholesky.t().dot(&cholesky);
        assert_eq!(
            path.shape(), &[2, steps],
            "path must be of shape ({}, {}), but is of shape {:?}",
            2, steps, path.shape()
        );

        assert!(
            (cholesky[[1,1]] - cholesky_variable).abs() < 1e-8,
            "Cholesky decomposition is not correct. Expected: {}, got: {}",
            cholesky_variable, cholesky[[1,1]]
        );

        // checking cholsky decomposition recovers the correlation matrix
        assert!(
            (recalc_cov - &correlation_matrix).mapv(|x| x.abs()).sum() < 0.1,
            "Cholesky decomposition does not recover the correlation matrix. \nExpected: {:?}, \ngot: {:?}",
            &correlation_matrix, cholesky.dot(&cholesky.t())
        );
    }
}

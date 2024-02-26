use crate::definitions::Real;
use ndarray::Array2;
use log::warn;

pub fn cholesky_decomposition(matrix: &Array2<Real>) -> Result<Array2<Real>, &'static str> {
    let mut msg = String::from("Cholesky decomposition is carried out by this custom function (utils::cholesky_decomposition)\n");
    msg.push_str("It is better to use a library for optimization, for example, ndarray-linalg crate has a cholesky function.\n");
    msg.push_str("In addition, check BLAS is being used. Currently disabled for multi developing environment");
    warn!("{}", msg);

    let n = matrix.nrows();
    if n != matrix.ncols() {
        return Err("Matrix must be square");
    }

    let mut l = Array2::<Real>::zeros((n, n));

    for i in 0..n {
        for j in 0..=i {
            let mut sum = 0.0;

            // Compute the sum of L[i, k] * L[j, k] for all k < j
            for k in 0..j {
                sum += l[[i, k]] * l[[j, k]];
            }

            if i == j {
                // The diagonal elements
                let diagonal = matrix[[i, i]] - sum;
                if diagonal <= 0.0 {
                    return Err("Matrix is not positive definite");
                }
                l[[i, i]] = diagonal.sqrt();
            } else {
                // The off-diagonal elements
                let value = (matrix[[i, j]] - sum) / l[[j, j]];
                l[[i, j]] = value;
            }
        }
    }

    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_cholesky_decomposition() {

        for i in 0..9
         {
            let rho = (i as Real) * 0.1;
            // 2 dim
            let matrix = arr2(&[[1.0, rho], [rho, 1.0]]);
            let l = cholesky_decomposition(&matrix).unwrap();
            assert!(
                (&l.dot(&l.t()) - &matrix).mapv(|x| x.abs()).sum() < 1e-7,
                "Cholesky decomposition failed for \nmatrix = \n{:?}\nbut L = \n{:?}\n and L*L^T = \n{:?}", 
                matrix, l, &l.dot(&l.t())
            );
            // 3 dim
            let matrix = arr2(&[[1.0, rho, rho], [rho, 1.0, rho], [rho, rho, 1.0]]);
            let l = cholesky_decomposition(&matrix).unwrap();
            assert!(
                (&l.dot(&l.t()) - &matrix).mapv(|x| x.abs()).sum() < 1e-7,
                "Cholesky decomposition failed for \nmatrix = \n{:?}\nbut L = \n{:?}\n and L*L^T = \n{:?}", 
                matrix, l, &l.dot(&l.t())
            );
            // 4 dim
            let matrix = arr2(&[[1.0, rho, rho, rho], [rho, 1.0, rho, rho], [rho, rho, 1.0, rho], [rho, rho, rho, 1.0]]);
            let l = cholesky_decomposition(&matrix).unwrap();
            assert!(
                (&l.dot(&l.t()) - &matrix).mapv(|x| x.abs()).sum() < 1e-6,
                "Cholesky decomposition failed for \nmatrix = \n{:?}\nbut L = \n{:?}\n and L*L^T = \n{:?}", 
                matrix, l, &l.dot(&l.t())
            );
        }
    }
}

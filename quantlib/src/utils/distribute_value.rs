use crate::definitions::Real;
use crate::utils::find_index::binary_search_index;
use anyhow::{Result, anyhow};

pub fn distribute_value_on_vector(
    vector: &mut Vec<Real>,
    value: Real,
    value_location: Real,
    vector_location: & Vec<Real>,
) -> Result<()> {
    let n = vector.len();
    if n != vector_location.len() {
        Err(anyhow!(
            "({}:{}) vector and vector_location must have the same length",
            file!(), line!()
        ))?;
    }

    if n == 0 {
        return Ok(()); // nothing to do
    }

    if value_location <= vector_location[0] {
        vector[0] += value;
    } else if value_location >= vector_location[n - 1] {
        vector[n - 1] += value;
    } else {
        let index = binary_search_index(&vector_location, value_location);
        let right_weight = (value_location - vector_location[index]) / (vector_location[index + 1] - vector_location[index]);
        let left_weight = 1.0 - right_weight;
        vector[index] += left_weight * value;
        vector[index + 1] += right_weight * value;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_distribute_value_on_vector() -> Result<()> {
        let mut vector = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        let value = 1.0;
        let value_location = 1.0;
        let vector_location = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        distribute_value_on_vector(&mut vector, value, value_location, &vector_location).unwrap();
        for i in 0..vector.len() {
            if i == 0 {
                assert!(
                    (vector[i] - 1.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else {
                assert!(
                    (vector[i] - 0.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            }
        }
        
        let value_location = 5.0;
        distribute_value_on_vector(&mut vector, value, value_location, &vector_location).unwrap();
        for i in 0..vector.len() {
            if i == 4 {
                assert!(
                    (vector[i] - 1.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else if i == 0 {
                assert!(
                    (vector[i] - 1.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else {
                assert!(
                    (vector[i] - 0.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            }
        }

        let value_location = 3.5;
        distribute_value_on_vector(&mut vector, value, value_location, &vector_location).unwrap();
        for i in 0..vector.len() {
            if i == 2 {
                assert!(
                    (vector[i] - 0.5).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else if i == 3 {
                assert!(
                    (vector[i] - 0.5).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else if i == 0 {
                assert!(
                    (vector[i] - 1.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else if i == 4 {
                assert!(
                    (vector[i] - 1.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            } else {
                assert!(
                    (vector[i] - 0.0).abs() < 1e-7,
                    "vector[{}] = {}",
                    i, vector[i]
                );
            }
        }
        Ok(())
    }
}

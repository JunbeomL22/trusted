/// Binary search for the index of a value in a sorted vector.
/// vec: The vector to search.
/// value: The value to search for.
/// Returns the index of the value in the vector such that vec[index] <= value < vec[index+1].
/// If the value is less than the first element, 0 is returned.
/// If the value is greater than or equal to the last element, the index of the last element is returned.
/// # Example
/// ```
/// use quantlib::utils::find_index::binary_search_index;
/// 
/// let vec = vec![1.0, 2.0, 6.0, 9.0, 11.0];
/// let index = binary_search_index(&vec, 0.5);
/// println!("{}", index); // 0
/// let index = binary_search_index(&vec, 5.0);
/// println!("{}", index); // 2
/// let index = binary_search_index(&vec, 10.0);
/// println!("{}", index); // 4
/// ```
pub fn binary_search_index<T: PartialOrd + Copy>(vec: &[T], value: T) -> usize {
    let mut low = 0;
    let mut high = if vec.is_empty() { 0 } else { vec.len() - 1 };

    while low <= high {
        let mid = (low + high) / 2;
        if vec[mid] == value {
            return mid;
        } else if vec[mid] < value {
            low = mid + 1;
        } else {
            if mid == 0 { break; } // Prevents underflow
            high = mid - 1;
        }
    }

    // Handles the case where the value is greater than all elements in the vector
    if low > vec.len() - 1 {
        return vec.len() - 1;
    }

    // Adjusts low to ensure the condition vec[index] <= value < vec[index+1]
    if vec[low] < value { 
        low 
    } else if low == 0 { 
        0 
    } else { 
        low - 1 
    }
}

/// vectorized bunary search index function for sorted input vector and sorted search vector
/// If the input length is less than 2, this function find each index by binary search and return the result
/// If the input length is bigger than 2, this function find the first index and the last index of the search vector by binary search 
/// and then find the index of the search vector by linear search between the first index and the last index
/// This function outperforms, of course, when the search_vec is dense and the input vector is large 
/// Note that this function does not check the input vector is sorted or not
/// If not, this function will return wrong result not panic
/// 
/// TODO: add an integer input to decide the iteration number to perform binary search
/// For example, if we perform binary search for the first, last, and middle, and then perform linear search for the two groups
/// For now, the iteration number is 2, only first and last index
/// If we have to deal with extremely large input vector, we can add the iteration number to perform binary search
/// # Example
/// ```
/// use quantlib::utils::find_index::vectorized_search_index_for_sorted_vector;
/// let vec = vec![1.0, 2.0, 6.0, 9.0, 11.0];
/// let search_vec = vec![0.5, 5.0, 10.0];
/// let index = vectorized_search_index_for_sorted_vector(&vec, &search_vec);
/// println!("{:?}", index); // [0, 2, 4]
/// ```
pub fn vectorized_search_index_for_sorted_vector<T: PartialOrd + Copy>(vec: &[T], search_vec: &[T]) -> Vec<usize> {
    let length = search_vec.len();
    let mut result = vec![0; length];
    if length <= 2 {
        for i in 0..length {
            result[i] = binary_search_index(vec, search_vec[i]);
        }
    } else {
        let first_index = binary_search_index(vec, search_vec[0]);
        let last_index = binary_search_index(vec, search_vec[search_vec.len()-1]);

        let mut j = first_index;
        for i in 0..length {
            if search_vec[i] < vec[first_index] {
                result[i] = first_index;
            } else if search_vec[i] >= vec[last_index] {
                result[i] = last_index;
            } else {
                while j < last_index {
                    if search_vec[i] >= vec[j] && search_vec[i] < vec[j+1] {
                        result[i] = j;
                        break;
                    }
                    j += 1;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::Real;

    #[test]
    fn test_binary_search_index() {
        let vec = vec![1, 3, 6, 9, 11];
        assert_eq!(binary_search_index(&vec, 0), 0);
        assert_eq!(binary_search_index(&vec, 1), 0);
        assert_eq!(binary_search_index(&vec, 2), 0);
        assert_eq!(binary_search_index(&vec, 3), 1);
        assert_eq!(binary_search_index(&vec, 4), 1);
        assert_eq!(binary_search_index(&vec, 5), 1);
        assert_eq!(binary_search_index(&vec, 6), 2);
        assert_eq!(binary_search_index(&vec, 7), 2);
        assert_eq!(binary_search_index(&vec, 8), 2);
        assert_eq!(binary_search_index(&vec, 9), 3);
        assert_eq!(binary_search_index(&vec, 10), 3);
        assert_eq!(binary_search_index(&vec, 11), 4);
        assert_eq!(binary_search_index(&vec, 12), 4);

        let vec: Vec<Real> = vec![1.0, 3.0, 6.0, 9.0, 11.0];
        assert_eq!(binary_search_index(&vec, 0.0), 0);
        assert_eq!(binary_search_index(&vec, 1.0), 0);
        assert_eq!(binary_search_index(&vec, 2.0), 0);
        assert_eq!(binary_search_index(&vec, 3.0), 1);
        assert_eq!(binary_search_index(&vec, 4.0), 1);
        assert_eq!(binary_search_index(&vec, 5.0), 1);
        assert_eq!(binary_search_index(&vec, 6.0), 2);
        assert_eq!(binary_search_index(&vec, 7.0), 2);
        assert_eq!(binary_search_index(&vec, 8.0), 2);
        assert_eq!(binary_search_index(&vec, 9.0), 3);
        assert_eq!(binary_search_index(&vec, 10.0), 3);
        assert_eq!(binary_search_index(&vec, 11.0), 4);
        assert_eq!(binary_search_index(&vec, 12.0), 4);

    }

    #[test]
    fn test_vectorized_binary_search_index_for_sorted_vector() {
        let vec: Vec<Real> = vec![1.0, 2.0, 6.0, 9.0, 11.0];

        let search_vec = vec![8.0];
        let index = vectorized_search_index_for_sorted_vector(&vec, &search_vec);
        assert_eq!(index, vec![2]);

        let search_vec = vec![3.0, 8.0];
        let index = vectorized_search_index_for_sorted_vector(&vec, &search_vec);
        assert_eq!(index, vec![1, 2]);

        let search_vec = vec![0.0, 0.0, 0.1, 1.0, 2.0, 6.0, 9.0, 10.0, 11.0, 12.0];
        let index = vectorized_search_index_for_sorted_vector(&vec, &search_vec);
        assert_eq!(index, vec![0, 0, 0, 0, 1, 2, 3, 3, 4, 4]);
    }
}

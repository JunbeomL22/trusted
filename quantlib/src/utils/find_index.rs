/// Binary search for the index of a value in a sorted vector.
/// vec: The vector to search.
/// value: The value to search for.
/// Returns the index of the value in the vector such that vec[index] <= value < vec[index+1].
/// If the value is less than the first element, 0 is returned.
/// If the value is greater than or equal to the last element, the index of the last element is returned.
/// # Example
/// ```
/// let vec = vec![1.0, 2.0, 6.0, 9.0, 11.0];
/// let index = binary_search_index(&vec, 0.5);
/// println!("{}", index); // 0
/// let index = binary_search_index(&vec, 5.0);
/// println!("{}", index); // 2
/// let index = binary_search_index(&vec, 10.0);
/// println!("{}", index); // 4
/// ```
pub fn binary_search_index<T: PartialOrd>(vec: &[T], value: T) -> usize {
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
    if vec[low] < value { low } else { if low == 0 { 0 } else { low - 1 } }
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

}

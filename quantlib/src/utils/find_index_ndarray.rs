use ndarray::Array1;

/// This is an ndarray version of the binary search index
pub fn binary_search_index_ndarray<T: PartialOrd + Copy>(arr: &Array1<T>, value: T) -> usize {
    let mut low = 0;
    let mut high = if arr.is_empty() { 0 } else { arr.len() - 1 };

    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] == value {
            return mid;
        } else if arr[mid] < value {
            low = mid + 1;
        } else {
            if mid == 0 { break; } // Prevents underflow
            high = mid - 1;
        }
    }

    // Handles the case where the value is greater than all elements in the array
    if low > arr.len() - 1 {
        return arr.len() - 1;
    }

    // Adjusts low to ensure the condition arr[index] <= value < arr[index+1]
    if arr[low] < value { low } else { if low == 0 { 0 } else { low - 1 } }
}

/// This is an ndarray version of the vectorized binary search index
pub fn vectorized_search_index_for_sorted_ndarray<T: PartialOrd + Copy>(arr: &Array1<T>, search_arr: &Array1<T>) -> Vec<usize> {
    let length = search_arr.len();
    let mut result = vec![0; length];
    if length <= 2 {
        for i in 0..length {
            result[i] = binary_search_index_ndarray(arr, search_arr[i]);
        }
    } else {
        let first_index = binary_search_index_ndarray(arr, search_arr[0]);
        let last_index = binary_search_index_ndarray(arr, search_arr[search_arr.len()-1]);

        let mut j = first_index;
        for i in 0..length {
            if search_arr[i] < arr[first_index] {
                result[i] = first_index;
            } else if search_arr[i] >= arr[last_index] {
                result[i] = last_index;
            } else {
                while j < last_index {
                    if search_arr[i] >= arr[j] && search_arr[i] < arr[j+1] {
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
    use ndarray::Array1;

    #[test]
    fn test_binary_search_index_ndarray() {
        let arr = Array1::from(vec![1.0, 2.0, 6.0, 9.0, 11.0]);
        let index = binary_search_index_ndarray(&arr, 10.0);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_vectorized_search_index_for_sorted_ndarray() {
        let arr = Array1::from(vec![1.0, 2.0, 6.0, 9.0, 11.0]);
        let search_arr = Array1::from(vec![0.5, 5.0, 10.0]);
        let index = vectorized_search_index_for_sorted_ndarray(&arr, &search_arr);
        assert_eq!(
            index, 
            vec![0, 1, 3],
            "\nSearching location of 0.5, 5.0, 10.0 in [1.0, 2.0, 6.0, 9.0, 11.0] should be [0, 2, 3], but got {:?}", 
            index
        );
    }
}
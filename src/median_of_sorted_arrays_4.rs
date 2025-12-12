    #[allow(dead_code)]
fn median_of_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged_array = Vec::new();
    let mut index_1 = 0;
    let mut index_2 = 0;
    if nums1.len() == 0 {
        merged_array = nums2;
    } else if nums2.len() == 0 {
        merged_array = nums1;
    } else {
        loop {
            let item_1 = nums1[index_1];
            let item_2 = nums2[index_2];
            if item_1 <= item_2 {
                merged_array.push(item_1);
                index_1 += 1;
            }
            if item_2 <= item_1 {
                merged_array.push(item_2);
                index_2 += 1;
            }
            if index_1 == nums1.len() {
                for item in nums2[index_2..].iter() {
                    merged_array.push(*item);
                }
                break;
            }
            if index_2 == nums2.len() {
                for item in nums1[index_1..].iter() {
                    merged_array.push(*item);
                }
                break;
            }
        }
    }
    let mid_array_index = merged_array.len() / 2;
    if merged_array.len() % 2 == 0 {
        (merged_array[mid_array_index - 1] + merged_array[mid_array_index]) as f64 / 2.0
    } else {
        merged_array[mid_array_index] as f64
    }
}

#[test]
fn test_median_of_sorted_arrays() {
    assert_eq!(median_of_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(median_of_sorted_arrays(vec![3], vec![2, 4, 5]), 3.5);
    assert_eq!(median_of_sorted_arrays(vec![3], vec![]), 3.0);
}

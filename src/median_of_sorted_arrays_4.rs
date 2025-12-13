#[allow(dead_code)]
fn median_of_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mid_array_index = (nums1.len() + nums2.len()) / 2;
    let is_even = (nums1.len() + nums2.len()) % 2 == 0;
    let mut index_1 = 0;
    let mut index_2 = 0;
    let mut previous: Option<i32> = None;
    loop {
        let item_1 = nums1.get(index_1);
        let item_2 = nums2.get(index_2);
        match (item_1, item_2) {
            (Some(item_1), Some(item_2)) => {
                let item = if item_1 <= item_2 {
                    index_1 += 1;
                    item_1
                } else {
                    index_2 += 1;
                    item_2
                };
                if index_1 + index_2 - 1 == mid_array_index {
                    return if is_even {
                        ((*item + previous.expect("Unreachable")) as f64) / 2.0
                    } else {
                        *item as f64
                    }
                }
                previous = Some(*item);
            }
            (Some(item_1), None) => {
                index_1 += 1;
                if index_1 + index_2 - 1 == mid_array_index {
                    return if is_even {
                        ((*item_1 + previous.expect("Unreachable")) as f64) / 2.0
                    } else {
                        *item_1 as f64
                    }
                }
                previous = Some(*item_1);
            }
            (None, Some(item_2)) => {
                index_2 += 1;
                if index_1 + index_2 - 1 == mid_array_index {
                    return if is_even {
                        ((*item_2 + previous.expect("Unreachable")) as f64) / 2.0
                    } else {
                        *item_2 as f64
                    }
                }
                previous = Some(*item_2);
            }
            (None, None) => {
                unreachable!()
            }
        }
    }
}

#[test]
fn test_median_of_sorted_arrays() {
    assert_eq!(median_of_sorted_arrays(vec![5], vec![]), 5.0);
    assert_eq!(median_of_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(median_of_sorted_arrays(vec![1, 3, 4], vec![2, 5]), 3.0);
    assert_eq!(median_of_sorted_arrays(vec![1, 3, 4, 5], vec![2]), 3.0);
    assert_eq!(median_of_sorted_arrays(vec![3, 4], vec![2, 5]), 3.5);
    assert_eq!(median_of_sorted_arrays(vec![3], vec![2, 4, 5]), 3.5);
}

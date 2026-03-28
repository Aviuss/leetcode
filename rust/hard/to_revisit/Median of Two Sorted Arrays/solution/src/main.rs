struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();


        0.0
    }
}

fn main() {
    
    println!("1Sol {:} (expected 2)", Solution::find_median_sorted_arrays(vec![1,3], vec![2]));
    println!("2Sol {:} (expected 2.5)", Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]));
    println!("2Sol {:} (expected 2.5)", Solution::find_median_sorted_arrays(vec![1,2,3,4], vec![10]));
}

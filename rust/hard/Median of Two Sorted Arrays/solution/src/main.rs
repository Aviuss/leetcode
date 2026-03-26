struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();

        let mut merged: Vec<i32> = Vec::with_capacity(nums1_len + nums2_len);

        let mut nums1_counter: usize = 0;
        let mut nums2_counter: usize = 0;
        let mut nums1_finish: bool = nums1_len == 0;
        let mut nums2_finish: bool = nums2_len == 0;

        let middle = (nums1_len  + nums2_len) / 2;
        for i in 0..(nums1_len+nums2_len) {
            
            if nums1_finish {
                merged.push(nums2[nums2_counter]);
                nums2_counter = nums2_counter + 1;
            } else if nums2_finish  {
                merged.push(nums1[nums1_counter]);
                nums1_counter = nums1_counter + 1;
            } else { // normal execution
                if nums1[nums1_counter] < nums2[nums2_counter] {
                    merged.push(nums1[nums1_counter]);
                    nums1_counter = nums1_counter + 1;
                } else {
                    merged.push(nums2[nums2_counter]);
                    nums2_counter = nums2_counter + 1;
                }
            }

            nums1_finish = nums1_counter == nums1_len;
            nums2_finish = nums2_counter == nums2_len;
        
            if i == middle {
                break;
            }
        }

        if (nums1_len + nums2_len) % 2 == 0 {
            return (merged[middle] + merged[middle - 1]) as f64 / 2.0;
        } else {
            return merged[middle] as f64;
        }
    }
}

fn main() {
    
    println!("1Sol {:} (expected 2)", Solution::find_median_sorted_arrays(vec![1,3], vec![2]));
    println!("2Sol {:} (expected 2.5)", Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]));
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
            

        vec![]
    }
}

fn main() {
    println!("Sol1 {:?}", Solution::two_sum(vec![2,7,11,15], 9));
    println!("Sol2 {:?}", Solution::two_sum(vec![3,2,4], 6));
    println!("Sol3 {:?}", Solution::two_sum(vec![3,3], 6));
    println!("Sol4 {:?}", Solution::two_sum(vec![0,4,3,0], 0));
}
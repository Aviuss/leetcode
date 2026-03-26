use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        
        for i in 0..nums.len() {
            let complement = target - nums[i];
            
            if let Some(ci) = map.get(&complement) {
                return vec![*ci as i32, i as i32]
            }

            map.insert(nums[i], i);
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
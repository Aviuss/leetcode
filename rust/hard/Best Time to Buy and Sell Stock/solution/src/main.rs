struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut max_profit: i32 = 0;
        let mut prev_lowest: i32 = prices[0]; 
        
        for i in 1..prices.len() {
            let profit = prices[i] - prev_lowest;
            if profit > max_profit {
                max_profit = profit;
            }
            if prices[i] < prev_lowest {
                prev_lowest = prices[i];
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sol1() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
    }

    #[test]
    fn sol2() {
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
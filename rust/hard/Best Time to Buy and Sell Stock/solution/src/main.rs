struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit: i32 = 0;

        for i in 0..prices.len() {
            for j in 1..(prices.len()) {
                if j <= i {
                    continue;
                }
                let profit = prices[j]-prices[i];
                if profit > max_profit {
                    max_profit = profit;
                }

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
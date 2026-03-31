struct Solution {}

impl Solution {
    fn sqrt(x: u32) -> u32 {
        if x <= 1 {
            return x;
        }
        let mut low: u32 = 1;
        let mut high: u32 = x-1;

        let mut curr_best_sol: u32 = 0;

        while low <= high {
            let middle: u32 = low + (high - low) / 2;
            let middle_pow: u64 = (middle  as u64) * (middle as u64);

            if middle_pow == x as u64 {
                return middle;
            } else if middle_pow > x as u64 {
                high = middle - 1;
            } else {
                low = middle + 1;
                curr_best_sol = middle;
            }
        }

        curr_best_sol
    }

    pub fn my_sqrt(x: i32) -> i32 {
        return Solution::sqrt(
            x.try_into().expect("Positive number should be provided")
        ).try_into().expect("Should fit as provided by leetcode")
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_4() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }
    
    #[test]
    fn sqrt_8() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn sqrt_16() {
        assert_eq!(Solution::my_sqrt(16), 4);
    }

    #[test]
    fn sqrt_17() {
        assert_eq!(Solution::my_sqrt(17), 4);
    }

    #[test]
    fn sqrt_15() {
        assert_eq!(Solution::my_sqrt(15), 3);
    }

    #[test]
    fn sqrt_18() {
        assert_eq!(Solution::my_sqrt(18), 4);
    }    #[test]
    
    #[test]
    fn sqrt_19() {
        assert_eq!(Solution::my_sqrt(19), 4);
    }

    #[test]
    fn sqrt_20() {
        assert_eq!(Solution::my_sqrt(20), 4);
    }

    #[test]
    fn sqrt_21() {
        assert_eq!(Solution::my_sqrt(21), 4);
    }

    #[test]
    fn sqrt_22() {
        assert_eq!(Solution::my_sqrt(22), 4);
    }

    #[test]
    fn sqrt_23() {
        assert_eq!(Solution::my_sqrt(23), 4);
    }

    #[test]
    fn sqrt_24() {
        assert_eq!(Solution::my_sqrt(24), 4);
    }

    #[test]
    fn sqrt_25() {
        assert_eq!(Solution::my_sqrt(25), 5);
    }

    #[test]
    fn sqrt_26() {
        assert_eq!(Solution::my_sqrt(26), 5);
    }

    #[test]
    fn sqrt_2147395599() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339)
    }

    #[test]
    fn sqrt_4294967295() {
        let x: u32 = 4294967295;
        assert_eq!(Solution::sqrt(x), 65535)
    }
}
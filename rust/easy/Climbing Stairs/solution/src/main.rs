struct Solution {}

impl Solution {
    fn factorial(n: i128) -> i128 {
        let mut sol = 1;
        for i in 2..(n+1) {
            sol = sol * i;
        }

        sol
    }

    fn newtons_factorial(n: i32, k: i32) -> i32 {
        if n < k {
            panic!("Newton symbol with n={:?}, k={:?} is invalid", n, k);
        }

        let n_minus_k = n - k;

        let (bigger, smaller) = if n_minus_k < k {
            (k, n_minus_k)
        } else {
            (n_minus_k, k)
        };

        let mut solution: i128 = 1;
        for i in (bigger+1)..(n+1) {
            solution = solution * i as i128; 
        }

        solution = solution / Solution::factorial(smaller as i128);

        solution.try_into()
            .expect("Factorial should fit to i32, as a task requires it do do so.")
    }

    pub fn climb_stairs(n: i32) -> i32 {
        if n < 0 {
            panic!("N is supposed to be non negative");
        }
        if n == 0 {
            return 0;
        }
        let n = n as u32;
        
        let mut solutions: i32 = 0;
        
        for L in 0..(n/2 +1) {
            if L == 0 {
                solutions = solutions + 1;
                continue;
            }

            let space: u32 = n - L*2;
            let one_placement_options: u32 = L + 1;
            // Stars and bars (combinatorics)
            let combinations = Solution::newtons_factorial(
                (space + one_placement_options - 1) as i32,
                (one_placement_options - 1) as i32
            );

            solutions = solutions + combinations;
        }

        solutions as i32
    }
}

fn main() {
    println!("Hello, world!");
    //Solution::climb_stairs(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn n_minus() {
        Solution::climb_stairs(-1);
    }

    #[test]
    fn n0() {
        assert_eq!(Solution::climb_stairs(0), 0);
    }

    #[test]
    fn n1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn n2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn n3() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn n4() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn factorial() {
        assert_eq!(Solution::factorial(5), 120);
    }

    #[test]
    fn newtons_factorial() {
        assert_eq!(Solution::newtons_factorial(5, 1), 5);
        assert_eq!(Solution::newtons_factorial(5, 2), 10);
        assert_eq!(Solution::newtons_factorial(5, 3), 10);
    }

    #[test]
    #[should_panic]
    fn newtons_factorial_panic() {
        Solution::newtons_factorial(10, 100);
    }
}
struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x >= 0 && x < 10 {
            return true;
        }
        let x = x as u32;

        let mut div_num: u64 = 10;
        let mut numbers: Vec<u8> = vec![(x % 10) as u8];
        loop {
            let division: u32 = (x as u64 / div_num).try_into()
                .expect("x is u32, so when divided, it will be smaller.");

            if division == 0 {
                break;
            }

            let digit = division % 10;
            numbers.push(
                (digit).try_into()
                    .expect("Is a value [0;10) so will fit to u8")
            );
            div_num = div_num * 10;
        }

        let mut i = 0;
        let mut j: usize = numbers.len()-1;

        loop {
            if numbers[i] != numbers[j] {
                return false;
            }
            i = i + 1;
            j = j - 1;
            if i >= j {
                return true;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_1410110141() {
        assert_eq!(Solution::is_palindrome(1410110141), true)
    }

    #[test]
    fn test_12321() {
        assert_eq!(Solution::is_palindrome(12321), true);
    }

    #[test]
    fn test_123321() {
        assert_eq!(Solution::is_palindrome(123321), true);
    }    #[test]
    
    fn test_123521() {
        assert_eq!(Solution::is_palindrome(123521), false);
    }

    #[test]
    fn test_minus_121() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

}
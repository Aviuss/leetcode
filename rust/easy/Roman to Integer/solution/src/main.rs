struct Solution {}


impl Solution {
    fn roman_char_to_number(c: &char) -> Option<i32> {
        match c {
            'M' => Some(1000),
            'D' => Some(500),
            'C' => Some(100),
            'L' => Some(50),
            'X' => Some(10),
            'V' => Some(5),
            'I' => Some(1),
            _ => None
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut numbers: Vec<i32> = Vec::new();

        for c in s.chars() {
            let c_up: char = c.to_ascii_uppercase();
            let number = Solution::roman_char_to_number(&c_up)
                .expect("Should be roman letter.");
            
            numbers.push(number);
        }

        let numbers_length = numbers.len();
        let mut result: i32 = 0;
        let mut skip_one: bool = false;
        for i in 0..numbers_length {
            if skip_one {
                skip_one = false;
                continue;
            }
            if i + 1 == numbers_length {
                result = result + numbers[i];
                continue;
            } 
            if (numbers[i] < numbers[i+1]) {
                result = result - numbers[i] + numbers[i+1];
                skip_one = true;
                continue;
            }
            if (numbers[i] == numbers[i+1]) {
                result = result + numbers[i];
                continue;
            }

            result = result + numbers[i];
        }


        result
    }
}

fn main() {
    println!("1. Sol odp: {:?}", Solution::roman_to_int("III".to_string()));
    println!("1. Sol odp: {:?}", Solution::roman_to_int("LVIII".to_string()));
    println!("1. Sol odp: {:?}", Solution::roman_to_int("MCMXCIV".to_string()));
    println!("1. Sol odp: {:?}", Solution::roman_to_int("XX".to_string()));
}


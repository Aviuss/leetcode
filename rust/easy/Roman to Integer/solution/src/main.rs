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
        let vec: Vec<i32> = s.chars()
            .map(|c:char|
                c.to_ascii_uppercase()
            ).map(|c:char| 
                Solution::roman_char_to_number(&c).expect("Should be roman letter.")
            ).collect();

        let mut result: i32 = 0;
        let mut skip_one = false;

        for i in 0..vec.len() {    
            if skip_one {
                skip_one = false;
                continue;
            }
            if i + 1 == vec.len() {
                result = result + vec[i];
                continue;
            } 
            if vec[i] < vec[i+1] {
                result = result - vec[i] + vec[i+1];
                skip_one = true;
                continue;
            }

            result = result + vec[i];
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


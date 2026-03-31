struct Solution {}

#[derive(PartialEq)]
enum OpenParentheses {
    OSmooth,
    OSquare,
    OCurly
}

enum CloseParentheses {
    CSmooth,
    CSquare,
    CCurly
}

enum OpenOrCloseParentheses {
    Open(OpenParentheses),
    Close(CloseParentheses),
}

impl Solution {
    fn str_to_parenteses(c: char) -> Option<OpenOrCloseParentheses> {
        match c {
            '(' => Some(OpenOrCloseParentheses::Open(OpenParentheses::OSmooth)),
            '{' => Some(OpenOrCloseParentheses::Open(OpenParentheses::OCurly)),
            '[' => Some(OpenOrCloseParentheses::Open(OpenParentheses::OSquare)),
            ')' => Some(OpenOrCloseParentheses::Close(CloseParentheses::CSmooth)),
            '}' => Some(OpenOrCloseParentheses::Close(CloseParentheses::CCurly)),
            ']' => Some(OpenOrCloseParentheses::Close(CloseParentheses::CSquare)),
            _ => None
        }
    }

    fn given_closing_which_open(ce: CloseParentheses) -> OpenParentheses {
        match ce {
            CloseParentheses::CCurly => OpenParentheses::OCurly,
            CloseParentheses::CSmooth => OpenParentheses::OSmooth,
            CloseParentheses::CSquare => OpenParentheses::OSquare
        }
    }

    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        
        let mut stack: Vec<Option<OpenParentheses>> = Vec::with_capacity(s.len() / 2);
        let mut next_stack_idx: usize = 0;
        
        for c in s.chars() {
            let p = Solution::str_to_parenteses(c);
            match p {
                None => {
                    return false;
                },
                Some(OpenOrCloseParentheses::Open(op)) => {
                    if stack.len() > next_stack_idx {
                        stack[next_stack_idx] = Some(op);
                    } else {
                        stack.push(Some(op));
                    }
                    next_stack_idx = next_stack_idx + 1;
                },
                Some(OpenOrCloseParentheses::Close(cp)) => {
                    if next_stack_idx == 0 {
                        return false;
                    }
                    let op = Solution::given_closing_which_open(cp);
                    if let Some(val) = &stack[next_stack_idx-1] && op == *val{
                        next_stack_idx = next_stack_idx - 1;
                    } else {
                        return false;
                    }
                }
            }
            
        }
        
        return next_stack_idx == 0;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::is_valid(String::from("([])")), true);
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }

}
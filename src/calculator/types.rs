#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
    Multipy,
    Divide,
    BracketOpen,
    BracketClose,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Element {
    Number(i64),
    Operator(Sign),
}

impl Sign {
    fn priority(&self) -> i32 {
        match self {
            Sign::Plus => 2,
            Sign::Minus => 2,
            Sign::Multipy => 3,
            Sign::Divide => 3,
            Sign::BracketOpen => 0,
            Sign::BracketClose => 0,
        }
    }

    pub fn from_char(c: char) -> Sign {
        match c {
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            '*' => Sign::Multipy,
            '/' => Sign::Divide,
            '(' => Sign::BracketOpen,
            ')' => Sign::BracketClose,
            _ => panic!("Unknown sign"),
        }
    }

    pub fn number_from_chars(chars: &Vec<char>) -> Element {
        let mut num = String::new();
        for c in chars {
            if c.is_digit(10) {
                num.push(*c)
            } else {
                break;
            }
        }
        let res = num.parse::<i64>().unwrap();
        Element::Number(res)
    }
    

    pub fn calculate(&self, a: i64, b: i64) -> i64 {
        match self {
            Sign::Plus => a + b,
            Sign::Minus => a - b,
            Sign::Multipy => a * b,
            Sign::Divide => a / b,
            _ => panic!("Unknown sign"),
        }
    }

    pub fn is_higher_or_equal_priority(&self, other: Sign) -> bool {
        let first_priority = self.priority();
        let second_priority = other.priority();
        first_priority >= second_priority
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::types::Sign;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_is_higher_or_equal_priority() {
        let test_data = vec![((Sign::Multipy, Sign::Plus), true), ((Sign::Plus, Sign::Multipy), false), ((Sign::Plus, Sign::Plus), true), ((Sign::Minus, Sign::Plus), true), ((Sign::Minus, Sign::Minus), true)];

        for ((first, second), expected) in test_data {
            let subject = first.is_higher_or_equal_priority(second);
            assert_eq!(subject, expected);
        }
    }

    #[test]
    fn test_calculation() {
        let test_data = vec![((Sign::Plus, 2, 3), 5), ((Sign::Minus, 2, 3), -1), ((Sign::Multipy, 2, 3), 6), ((Sign::Divide, 2, 3), 0)];

        for ((sign, first, second), expected) in test_data {
            let subject = sign.calculate(first, second);
            assert_eq!(subject, expected);
        }
    }

    #[test]
    fn test_parse_numbers() {
        let test_data = vec![(vec!['2', '0', '1'], Element::Number(201)), (vec!['2', '0', '1', '0'], Element::Number(2010))];

        for (data, expected) in test_data {
            let subject = Sign::number_from_chars(&data);
            assert_eq!(subject, expected);
        }
    }
}

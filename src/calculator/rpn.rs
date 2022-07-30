use super::types::{Element, Sign};


pub fn parse(expression: String) -> Vec<Element> {
    let mut res = vec![];
    let mut stack = vec![];
    for sign in expression.chars() {
        match sign {
            '0'..='9' => {
                let number = sign.to_digit(10).unwrap();
                res.push(Element::Number(number as i32));
            },
            '+' | '-' | '*' | '/' => {
                let sign = Sign::from_char(sign);
                if stack.is_empty() {
                    stack.push(sign);
                } else {
                    let last = stack.pop().unwrap();
                    if sign.is_higher_or_equal_priority(last) {
                        stack.push(last);
                        stack.push(sign);
                    } else {
                        let mut last_sign = stack.pop().unwrap();
                        while last_sign.is_higher_or_equal_priority(sign) {
                            res.push(Element::Operator(last_sign));
                            last_sign = stack.pop().unwrap();
                        }
                        stack.push(sign);
                    }
                }
            },
            '(' => {
                stack.push(Sign::BracketOpen);
            },
            ')' => {
                let mut last_sign = stack.pop().unwrap();
                while last_sign != Sign::BracketOpen {
                    res.push(Element::Operator(last_sign));
                    last_sign = stack.pop().unwrap();
                }
            },
            _ => panic!("Unknown sign"),
        }
    }

    for stack_element in stack.into_iter() {
        res.push(Element::Operator(stack_element));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::calculator::types::Sign;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_calculation() {
        let test_data = vec![
                ("(2+3)*5", vec![Element::Number(2), Element::Number(3), Element::Operator(Sign::Plus), Element::Number(5), Element::Operator(Sign::Multipy)]),
                ("2*(5+2)", vec![Element::Number(2), Element::Number(5), Element::Number(2), Element::Operator(Sign::Plus), Element::Operator(Sign::Multipy)]),
            ];

        for (data, expected) in test_data {
            let subject = parse(data.to_owned());
            assert_eq!(subject, expected, "we are testing rpn notation parsong with {} and {:?}", data, expected);
        }
    }
}

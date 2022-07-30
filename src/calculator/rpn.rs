use super::types::{Element, Sign};

pub fn parse(expression: String) -> Vec<Element> {
    let mut res = vec![];
    let mut stack = vec![];
    for sign in expression.chars().into_iter() {
        match sign {
            '0'..='9' => {
                let number = sign.to_digit(10).unwrap();
                stack.push(Element::Number(number as i32));
            },
            '+' | '-' | '*' | '(' | ')' => {
                let sign = Sign::from_char(sign);
                stack.push(Element::Operator(sign));
            },
            _ => panic!("Unknown sign"),
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use crate::calculator::types::Sign;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_calculation() {
        let test_data = vec![("(2+3)*5", vec![Element::Number(2), Element::Number(3), Element::Operator(Sign::Plus), Element::Number(5), Element::Operator(Sign::Multipy)])];

        for (data, expected) in test_data {
            let subject = parse(data.to_owned());
            assert_eq!(subject, expected);
        }
    }
}

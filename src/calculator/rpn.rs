use super::types::{Element, Sign};


pub fn parse(expression: Vec<Element>) -> Vec<Element> {
    let mut res = vec![];
    let mut stack: Vec<Sign> = vec![];
    for element in expression {
        match element {
            Element::Number(_) => {
                res.push(element);
            },
           Element::Operator(sign @ Sign::Divide) |  Element::Operator(sign @ Sign::Multipy) | Element::Operator(sign @  Sign::Plus) | Element::Operator(sign @  Sign::Minus) => {
                if stack.is_empty() {
                    stack.push(sign);
                } else {
                    let last = stack.pop().unwrap();
                    if sign.is_higher_or_equal_priority(last) {
                        stack.push(last);
                        stack.push(sign);
                    } else {
                        let mut last_sign = last;
                        while last_sign.is_higher_or_equal_priority(sign) {
                            res.push(Element::Operator(last_sign));
                            last_sign = stack.pop().unwrap();
                        }
                        stack.push(sign);
                    }
                }
            },
            Element::Operator(sign @ Sign::BracketOpen) => {
                stack.push(sign);
            },
            Element::Operator(Sign::BracketClose) => {
                let mut last_sign = stack.pop().unwrap();
                while last_sign != Sign::BracketOpen {
                    res.push(Element::Operator(last_sign));
                    last_sign = stack.pop().unwrap();
                }
            },
        }
    }
    if !stack.is_empty() {
        let mut element = stack.pop();
        while element.is_some() {
            res.push(Element::Operator(element.unwrap()));
            element = stack.pop();
        }
    }
    res
}


pub fn calculate (elements: Vec<Element>) -> i32 {
    let mut stack = vec![];
    for element in elements {
        match element {
            Element::Number(n) => {
                stack.push(n);
            },
            Element::Operator(sign) => {
                let a = stack.pop().unwrap_or(1);
                let b = stack.pop().unwrap_or(1);
                let result = sign.calculate(a, b);
                stack.push(result);
            }
        }
    }
    stack.pop().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::calculator::{types::Sign, parser};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_parsing() {
        let test_data = vec![
                ("(2+3)*5", vec![Element::Number(2), Element::Number(3), Element::Operator(Sign::Plus), Element::Number(5), Element::Operator(Sign::Multipy)]),
                ("2*(5+2)", vec![Element::Number(2), Element::Number(5), Element::Number(2), Element::Operator(Sign::Plus), Element::Operator(Sign::Multipy)]),
                ("3+2*6", vec![Element::Number(3), Element::Number(2), Element::Number(6), Element::Operator(Sign::Multipy), Element::Operator(Sign::Plus)]),
                ("2+2*2", vec![Element::Number(2), Element::Number(2), Element::Number(2), Element::Operator(Sign::Multipy), Element::Operator(Sign::Plus)]),
            ];

        for (data, expected) in test_data {
            let elems = parser::parse_to_elements(data.to_owned());
            let subject = parse(elems);
            assert_eq!(subject, expected, "we are testing rpn notation parsong with {} and {:?}", data, expected);
        }
    }


    #[test]
    fn test_calculation() {
        let test_data = vec![
                (vec![Element::Number(2), Element::Number(3), Element::Operator(Sign::Plus), Element::Number(5), Element::Operator(Sign::Multipy)], 25),
                (vec![Element::Number(2), Element::Number(5), Element::Number(2), Element::Operator(Sign::Plus), Element::Operator(Sign::Multipy)], 14),
            ];

        for (data, expected) in test_data {
            let subject = calculate(data);
            assert_eq!(subject, expected, "we are testing rpn notation calculating with expected result {:?}", expected);
        }
    }
}

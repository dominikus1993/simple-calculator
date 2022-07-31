use super::types::{Element, Sign};



pub fn clean(input: &str) -> String {
    input.replace(' ', "")
}

pub fn parse_to_elements(expression: String) -> Vec<Element> {
    let mut res = vec![];
    let mut nums = vec![];
    for c in expression.chars() {
        match c {
            '0'..='9' => {
                nums.push(c);
            },
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !nums.is_empty() {
                    res.push(Sign::number_from_chars(&nums));
                    nums.clear();
                }
                res.push(Element::Operator(Sign::from_char(c)));
            },
            _ => panic!("Unknown sign"),
        }
    }
    if !nums.is_empty() {
        res.push(Sign::number_from_chars(&nums));
        nums.clear();
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::calculator::types::Sign;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_clean() {
        let test_data = vec![("2 * 2", "2*2"), ("2 + 2", "2+2")];

        for (data, expected) in test_data {
            let subject = clean(data);
            assert_eq!(subject, expected);
        }
    }

    #[test]
    fn test_parse_to_elements() {
        let test_data = vec![("2*2", vec![Element::Number(2), Element::Operator(Sign::Multipy), Element::Number(2)]), ("10+21", vec![Element::Number(10), Element::Operator(Sign::Plus), Element::Number(21)])];

        for (data, expected) in test_data {
            let subject = parse_to_elements(data.to_owned());
            assert_eq!(subject, expected);
        }
    }

}

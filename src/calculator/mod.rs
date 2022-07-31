mod types;
mod parser;
mod rpn;

pub fn calc(expression: &str) -> i64 {
    let clean_text = parser::clean(expression);
    let elements = parser::parse_to_elements(clean_text);
    let rpn_not = rpn::parse(elements);
    rpn::calculate(rpn_not)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculation() {
        let test_data = vec![("2*2", 4), ("2+2", 4), ("2-2", 0), ("2/2", 1), ("2+2*2", 6), ("2*2+2", 6), ("2 + 5", 7), ("21+37", 58)];

        for (data, expected) in test_data {
            let subject = calc(data);
            assert_eq!(subject, expected, "Failed for {}", data);
        }
    }
}

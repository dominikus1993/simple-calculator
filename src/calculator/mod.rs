mod types;
mod parser;
mod rpn;

fn calc(expression: &str) -> i32 {
    let clean_text = parser::clean(expression);
    let rpn_not = rpn::parse(clean_text);
    return 0;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculation() {
        let test_data = vec![("2*2", 4), ("2+2", 4), ("2-2", 0), ("2/2", 1), ("2+2*2", 6), ("2 + 5", 7)];

        for (data, expected) in test_data {
            let subject = calc(data);
            assert_eq!(subject, expected);
        }
    }
}

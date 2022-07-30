

pub fn clean(input: &str) -> String {
    input.replace(' ', "")
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculation() {
        let test_data = vec![("2 * 2", "2*2"), ("2 + 2", "2+2")];

        for (data, expected) in test_data {
            let subject = clean(data);
            assert_eq!(subject, expected);
        }
    }
}

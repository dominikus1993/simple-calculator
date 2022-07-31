mod calculator;

fn main() {
    let expression = "2137 + 69 * 2";
    let result = calculator::calc(expression);
    println!("Result of expression = {} is {}", expression, result);
}

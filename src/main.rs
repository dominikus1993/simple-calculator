mod calculator;

fn main() {
    let expression = "2137 + 420 * 69";
    let result = calculator::calc(expression);
    println!("Result of expression = {} is {}", expression, result);
}

use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("Calculate ('quit' to exit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("error unnable to read user input");

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        let input = input.replace(" ", "");
        let mut numbers = Vec::new();
        let mut operators = Vec::new();
        let mut num = String::new();

        for c in input.chars() {
            if c.is_digit(10) || c == '.' {
                num.push(c);
            } else {
                if !num.is_empty() {
                    numbers.push(num.parse::<f64>().unwrap());
                    num.clear();
                }
                operators.push(c);
            }
        }
        if !num.is_empty() {
            numbers.push(num.parse::<f64>().unwrap());
        }

        if numbers.len() != operators.len() + 1 {
            println!("Invalid input");
            continue;
        }

        let mut result = numbers[0];
        for i in 0..operators.len() {
            let operator = operators[i];
            let num = numbers[i + 1];
            result = match operator {
                '+' => result + num,
                '-' => result - num,
                '*' => result * num,
                '/' => {
                    if num == 0.0 {
                        println!("Error: Zero division");
                        break;
                    }
                    result / num
                }
                _ => {
                    println!("Invalid operator: {}", operator);
                    break;
                }
            };
        }

        println!("Result: {}", result);
    }
}
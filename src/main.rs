use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        // stop execution if there is are not enough arguments
        eprintln!("Supply 3 arguments: [num1] [operation] [num2]");
        return;
    }

    let number1: f64 = match (&args[1]).parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("First argument is an invalid number");
            return;
        }
    };

    let number2: f64 = match (&args[3]).parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Third argument is an invalid number");

            return;
        }
    };

    let operation = &args[2];

    return match operation.as_str() {
        "+" => println!("{}", number1 + number2),
        "-" => println!("{}", number1 - number2),
        "*" => println!("{}", number1 * number2),
        "/" => println!("{}", number1 / number2),
        _ => eprintln!("Invalid operation"),
    };
}

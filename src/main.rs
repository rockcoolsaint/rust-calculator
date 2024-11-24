use std::io;

fn main() {
    let result: f64;

    println!("Enter the first number: ");
    let x: f64 = input_parser();

    if f64::is_nan(x) {
        println!("Invalid input!");
        return;
    }

    println!("Enter the second number: ");
    let y: f64 = input_parser();

    if f64::is_nan(y) {
        println!("Invalid input!");
        return;
    }

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: ");

    let op: f64 = input_parser();

    if f64::is_nan(op) {
        println!("Invalid input!");
        return;
    }

    // let op: i32 = op as i32; alternative type casting directly in match block below

    result = match op as i32 {
        1 => x + y,
        2 => x - y,
        3 => x * y,
        4 => x / y,
        _ => {
            println!("Invalid selection");
            return;
        }
    };

    println!("The result is: {result}");
}

fn input_parser() -> f64 {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Invalid Input");
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return f64::NAN;
        }
    };

    return x;
}

use std::io;

fn main() {
    println!("---------Welcome to Calculator--------- ");

    println!("Enter first number ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();

    // Match se error handle kar rahe hain (agar number na ho to crash na kare)
    let num1: i32 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Number daalo bhai!");
            return;
        }
    };

    println!("Enter your second number ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();

    let num2: i32 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Number daalo bhai!");
            return;
        }
    };

    println!("Choose Op +, -, *, /");
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    if op == "+" {
        println!("Result {}", num1 + num2);
    } else if op == "-" {
        println!("Result {}", num1 - num2);
    } else if op == "*" {
        println!("Result {}", num1 * num2);
    } else if op == "/" {
        if num2 == 0 {
            println!("0 se divide nahi kar sakte bhai!");
        } else {
            println!("Result {}", num1 / num2);
        }
    } else {
        println!("Galat operation!");
    }
}

use std::io::Write;
mod arithremetic ;
use arithremetic::*;
pub fn minput() -> (i64, i64) {
    let mut num1 = String::new();
    let mut num2 = String::new();
    print!("Enter Number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num1).expect("Huh! Somethings Wroong...... with num1");
    print!("Enter Second Number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num2).expect("Huh! Somethings Wroong...... with num2");
    let num1: i64 = num1.trim().parse().expect("Enter a number!");
    let num2: i64 = num2.trim().parse().expect("Enter a number!");
    (num1, num2)
}
pub fn sinput() -> i64 {
    let mut num = String::new();
    print!("Enter Number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num).expect("Huh! Somethings Wroong...... with num1");
    let num: i64 = num.trim().parse().expect("Enter a number!");
    num
}
fn main() {
    let mut operator = String::new();
    println!("Rusty :D");
    loop {
        print!("Enter an operator: ");
        std::io::stdout().flush().unwrap();
        operator.clear();
        std::io::stdin().read_line(&mut operator).expect("Use one of these: + , - , * , / , ^ , // , % , | :D ");
        match operator.trim() {
        "+" => add(),
        "-" => sub(),
        "*" => multi(),
        "/" => div(),
        "//"=> remain(),
        "%" => prcnt(),
        "^" => expo(),
        "|" => absval(),
         _  => println!("Use one of these: + , - , * , / , ^ , // , % , | :D "),
        }
    }
}

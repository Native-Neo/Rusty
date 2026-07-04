use std::io; //alternative to C++'s "#include <iostream> "
use std::io::Write;
fn minput() -> (i64, i64) {
    let mut num1 = String::new();
    let mut num2 = String::new();
    print!("Enter Number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num1).expect("Huh! Somethings Wroong...... with num1");
    print!("Enter Second Number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num2).expect("Huh! Somethings Wroong...... with num2");
    let num1: i64 = num1.trim().parse().expect("Enter a number!");
    let num2: i64 = num2.trim().parse().expect("Enter a number!");
    (num1, num2)
}
fn sinput() -> i64 {
    let mut num = String::new();
    print!("Enter Number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut num).expect("Huh! Somethings Wroong...... with num1");
    let num: i64 = num.trim().parse().expect("Enter a number!");
    num
}
fn add() {
    let (num1, num2) = minput();
    let res = num1 + num2;
    println!("Result: {}", res);
}
fn sub() {
    let (num1, num2) = minput();
    let res = num1 - num2;
    println!("Result: {}", res);

}
fn multi() {
    let (num1, num2) = minput();
    let res = num1 * num2;
    println!("Result: {}", res);
}
fn div() {
    let (num1, num2) = minput();
    let res = num1 / num2;
    println!("Result: {}", res);
}
fn remain() {
    let (num1, num2) = minput();
    let res = num1 % num2;
    println!("Result: {}", res);
}
fn prcnt() {
    let (num1, num2) = minput();
    let res = (num1 as f64 / num2 as f64 ) * 100.0 ;
    println!("{num1} Is {res}% of {num2}");
}
fn expo() {
    let (base, mut power) = minput();
    let usedpower = power;
    let mut res = 1;
    while power > 0 {res *= base;
        power -= 1;

    };
    println!("{base} ^ {usedpower} Is {res}");
}
fn absval() {
    let num = sinput();
    let res = num.abs();
    println!("Absolute Value: {res}");
}
fn main() {
    let mut operator = String::new();
    println!("Rusty :D");
    loop {
        print!("Enter an operator: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut operator).expect("Use one of these: + , - , * , / :D ");
        if operator.trim() == "+"  {
            operator.clear();
            add();
        }
        else if operator.trim() == "-" {
            operator.clear();
            sub();
        }
        else if operator.trim() == "*" {
            operator.clear();
            multi();
        }
        else if operator.trim() == "/" {
            operator.clear();
            div();
        }
        else if operator.trim() == "//" {
            operator.clear();
            remain();
        }
        else if operator.trim() == "%" {
            operator.clear();
            prcnt();
        }
        else if operator.trim() == "^" {
            operator.clear();
            expo();
        }
        else if operator.trim() == "|" {
            operator.clear();
            absval();
        }
        else {break;}
    }
}

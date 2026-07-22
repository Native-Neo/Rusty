use crate::{sinput, minput};
pub fn add() {
    let (num1, num2) = minput();
    let res = num1 + num2;
    println!("Result: {}", res);
}
pub fn sub() {
    let (num1, num2) = minput();
    let res = num1 - num2;
    println!("Result: {}", res);

}
pub fn multi() {
    let (num1, num2) = minput();
    let res = num1 * num2;
    println!("Result: {}", res);
}
pub fn div() {
    let (num1, num2) = minput();
    let res = num1 / num2;
    println!("Result: {}", res);
}
pub fn remain() {
    let (num1, num2) = minput();
    let res = num1 % num2;
    println!("Result: {}", res);
}
pub fn prcnt() {
    let (num1, num2) = minput();
    let res = (num1 as f64 / num2 as f64 ) * 100.0 ;
    println!("{num1} Is {res}% of {num2}");
}
pub fn expo() {
    let (base, mut power) = minput();
    let usedpower = power;
    let mut res = 1;
    while power > 0 {res *= base;   power -= 1; };
    println!("{base} ^ {usedpower} Is {res}");
}
pub fn absval() {
    let num = sinput();
    let res = num.abs();
    println!("Absolute Value: {res}");
}

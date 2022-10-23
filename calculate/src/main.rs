use std::io;
fn main() {
    let mut counter: i32 = 0;
    let counter: i32 = loop {
        let o: char = get_operator();
        if o == 'q' {
            break counter;
        } else {
            let x: i32 = get_int("num1");
            let y: i32 = get_int("num2");
            println!("{}", calculate(x, y, o));
            counter += 1;
        }
    };
    println!("performed {} calculations", counter)
}
fn get_int(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("couldnt read input");
    let input: i32 = input.trim().parse().expect("couldnt convert to int");
    return input;
}
fn get_operator() -> char {
     println!("operator (+,-,*,/,q(uit))");
     let mut operator: String = String::new();
     io::stdin()
         .read_line(&mut operator)
         .expect("couldnt read input");
     let operator: char = operator.chars().next().unwrap();
     operator
 }
fn calculate(num1: i32, num2: i32, operator: char) -> i32 {
    match operator {
        '+' => num1+num2,
        '-' => num1-num2,
        '*' => num1*num2,
        '/' => num1/num2,
        _ => { for i in ['+', '-', '*', '/'].iter() { println!("{}", calculate(num1, num2, *i)); } 0 },
    }
}

use std::io::stdin;
fn main() {
    let n1 = get_input();
    let n2 = get_input();
    let result: Result<f32, String> = div(n1, n2);
    match result {
        Result::Ok(num) => println!("{}" ,num),
        Result::Err(err) => println!("{}", err),
    }
}
fn div(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by 0".to_string());
    }
    Result::Ok(a as f32 / b as f32)
}
fn get_input () -> i32 {
    let mut s: String = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Couldnt read input");
    let n: i32 = s
                .trim()
                .parse()
                .expect("Couldnt convert to number");
    n
}

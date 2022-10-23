fn main() {
    let mut inp: String = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("couldnt read");
    let inp: i32 = inp
        .trim()
        .parse()
        .expect("couldnt convert to integer");
    if (inp - 6) % 4 == 0 {
        println!("1");
        println!("{}", (inp - 6)/4 as i32);
    } else {
        println!("0");
    }
}

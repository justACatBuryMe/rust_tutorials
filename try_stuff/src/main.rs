fn main() {
    let s: &str = "12:34";
    let a: i32 = s.chars().take(2).collect::<String>().parse::<i32>().unwrap();
}

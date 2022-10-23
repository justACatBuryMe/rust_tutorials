fn main() {
    let mut a: String = String::from("asdf qwer");
    let first: &str = first_word(&a);
    println!("{}, {},", a, first);
    borrows_and_mutates(&mut a);
}
fn takes_ownership (x: String) {
    println!("{}", x);
}
fn takes_and_returns_ownership (x: String) -> String {
    println!("{}", x);
    x
}
fn copies_integer (x: i32) {
    println!("{}", x)
}
fn borrows (x: &String) {
    println!("{}", x)
}
fn borrows_and_mutates (x: &mut String) {
    println!("{}", x);
    x.push_str("qwer");
}
fn first_word (s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &chr) in bytes.iter().enumerate() {
        if chr == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

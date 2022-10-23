use std::collections::HashMap;

fn main() {
    let a: String = "asdfqweradsfasdf".to_string();
    let mut count: HashMap<char, i32> = HashMap::new();
    for i in a.chars() {
        let b: &mut i32 = count.entry(i).or_insert(0);
        *b += 1;
    }
    println!("{:?}", count);
    let count: [i32; 4] = [2,3,4,5];
    println!("{:?}", count);
}

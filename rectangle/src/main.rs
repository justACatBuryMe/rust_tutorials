use std::io;

fn main() {
    let rect1: Rectangle = get_rectangle();
    let rect2: Rectangle = get_rectangle();
    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    } else {
        println!("rect1 cannot hold rect2");
    }
    let square: Rectangle = Rectangle::square(5);
    println!("{}", square.can_hold(&rect1));
}
fn input_number (prompt: &str) -> i32 {
    println!("{}:", prompt);
    let mut num: String = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("couldnt get input");
    num
        .trim()
        .parse()
        .expect("couldnt convert to int")
}
fn get_rectangle () -> Rectangle {
    let width: i32 = input_number("width");
    let height: i32 = input_number("height");
    Rectangle { width, height }
}
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn area (&self) -> i32 {self.width * self.height}
    fn can_hold (&self, other: &Rectangle) -> bool {
        if self.area() < other.area() {
            return false;
        }
        if self.height >= other.height && self.width >= other.width{
            return true;
        } else if self.height >= other.width && self.width >= other.height{
            return true;
        } else { return false; }
    }
}
impl Rectangle {
    fn square (size: i32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

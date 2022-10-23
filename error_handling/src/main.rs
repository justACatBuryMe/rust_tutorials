use std::error::Error;
use std::fs::File;
use std::io::Read;
fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("poem.txt")?;
    let mut text: String = String::new();
    match f.read_to_string(&mut text){
        Ok(_) => {},
        Err(error) => panic!("at the disco {}", error),
    };
    println!("{}", text);
    println!("---");
    let mut f = File::open("poem.txt")?;
    let mut vettext: Vec<u8> = Vec::new();
    match f.read_to_end(&mut vettext) {
        Ok(_) => {},
        Err(error) => panic!("at the disco {}", error),
    }
    let mut vettextchar: Vec<char> = Vec::new();
    for i in &vettext {
        vettextchar.push(*i as char);
    }
    println!("{:?}", vettextchar);
    println!("{:?}", vettext);
    Ok(())
}

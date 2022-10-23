fn main() {
    let gorilla: Animal = Animal{
        species: String::from("Human"),
        date_of_invention: 2012,
        cost_per_fursuit: 12000,
        bipedal: true,
    };
    let panda: Animal = create_animal(String::from("Panda"), 2500, false);
    let koala: Animal = Animal { species: String::from("Koala"),
    date_of_invention: -2323,
    ..panda };
    struct colorRGB(u8, u8, u8);
    let red: colorRGB = colorRGB(255, 255, 255);
}
fn create_animal (species: String, cost_per_fursuit: u32, bipedal: bool) -> Animal {
    Animal {
        species,
        cost_per_fursuit,
        date_of_invention: 2020,
        bipedal: true,
    }
}
struct Animal {
    species: String,
    date_of_invention: i32,
    cost_per_fursuit: u32,
    bipedal: bool,
}

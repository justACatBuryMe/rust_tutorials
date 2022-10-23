fn main() {
    println!("hello world");
}
// fn main() {
//     let map: Vec<Vec<i32>> = Vec::from([Vec::from([1,2,1]),
//                                         Vec::from([3,8,3]),
//                                         Vec::from([5,3,1])]);
//     find_path(map);
// }
// fn find_path(map: Vec<Vec<i32>>) {
//     let mut max: i32 = i32::MAX;
//     let mut hikers: Vec<Hiker> = Vec::new();
//     hikers.push(Hiker::new(0, 0));
//     for mut hiker in hikers {
//         let (x, y) = (hiker.x, hiker.y);
//         let (mut hiker_x,mut  hiker_y) = hiker.reproduce();
//         hiker_x.climb(map[y][x] - map[hiker_y][hiker_x]);
//     }
// }
// struct Hiker {
//     x: i32,
//     y: i32,
//     steps: i32,
//     dead: bool,
// }
// impl Hiker {
//     fn climb (&mut self, steps: i32) {
//         self.steps += steps;
//     }
//     fn suicide (&mut self) {
//         self.dead = true
//     }
//     fn reproduce (&mut self) -> (Hiker, Hiker) {
//         self.suicide();
//         let new_x: Hiker = Hiker::new(self.x+1, self.y);
//         let new_y: Hiker = Hiker::new(self.x, self.y+1);
//         return (new_x, new_y);
//     }
//     fn new (x: i32, y: i32) -> Hiker {
//         return Hiker { x, y, steps: 0, dead: true };
//     }
// }

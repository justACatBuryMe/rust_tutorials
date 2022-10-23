mod front {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order(){}
        pub fn serve_order(){}
    }
}
fn cook_order() {}
mod back {
    fn fix_incorrect_order() {
        super::cook_order();
    }
}
pub fn eat_at_restaurant() {
    front::hosting::add_to_waitlist();
}

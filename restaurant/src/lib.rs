//create pub module for front_of_house operations
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(name: String){
            println!("{}, {}", String::from("You have been added to the waitlist"), name);
        }
        fn seat_at_table(){
            println!("{}", String::from("Follow me, your table is ready."));
        }
    }
    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist(String::from("Chris"));
    //relative path
    front_of_house::hosting::add_to_waitlist(String::from("Shunelett"));
}
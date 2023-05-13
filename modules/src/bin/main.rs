//use lib modules from package binary

//use modules::front_of_house::hosting;
//use modules::eat_at_restaurant;

//illustrate use of nested path import syntax
use modules::{eat_at_restaurant, front_of_house::hosting};

fn main() {
    hosting::add_to_waitlist(String::from("Christopher"));
    eat_at_restaurant();
}
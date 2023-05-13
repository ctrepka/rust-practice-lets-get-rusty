//use lib modules from package binary

use restaurant::front_of_house::hosting;
use restaurant::eat_at_restaurant;

fn main() {
    hosting::add_to_waitlist(String::from("Christopher"));
    eat_at_restaurant();
}
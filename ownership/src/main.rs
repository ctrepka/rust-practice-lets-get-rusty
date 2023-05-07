use std::slice;

fn main() {
    ownership_rules();

    ownership_1(String::from("hello"));
    ownership_2();
    ownership_3();

    println!("\nSTRING SLICE");
    // using String coerced to string slice
    println!("First word is: {}", slice_first_word(&String::from("FIRST WORD IS...")));
    // using string slice
    println!("Firest word is: {}", slice_first_word("Testing tester 123"));
}

fn ownership_rules() {
    println!("\n
1. Each value in rust has a variable that's called it's owner. \n
2. There can only be one owner at a time. \n
3. When the owner goes out of scope, the value is dropped.");
}

fn ownership_1(some_string: String) {
    println!("\nOWNERSHIP 1");
    println!("This works because var some_string takes ownership. some_string is: {}", some_string);
}

fn ownership_2() {
    println!("\nOWNERSHIP 2");
    let s1 = String::from("hello");
    println!("s1 value {}", s1);
    let s2 = s1;

    println!("//println!(s1); //fails bc moved to s2, value not copied, no pointer");
    println!("s2 value {}", s2);
}

fn ownership_3() {
    println!("\nOWNERSHIP 3");
    {
        let s = "hello";
        //do stuff with s
        println!("{}", s);
    } // this scope is over, s no longer exists
    // println!("{}", s); //throws error, s does not exist
    {

    }
}

fn slice_first_word(s: &str) -> &str {
    //break string to bytes array
    let bytes = s.as_bytes();
    //iterate through bytes
    for (i, &item) in bytes.iter().enumerate() {
        //the first time byte is equal to ' ', return reference to string slice from 0 index to i index
        if item == b' ' {
            return &s[0..i];
        }
    }
    //return all of new string
    &s[..]
}
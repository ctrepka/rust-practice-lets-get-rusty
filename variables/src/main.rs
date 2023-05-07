fn main() {
    let x: i32 = 5;
    println!("The value of x is {}", x);

    let x = "6";
    println!("The value of x is {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    println!("{}", SUBSCRIBER_COUNT);

    let tup = ("Let's get rusty", SUBSCRIBER_COUNT);

    let (channel, subcount) = tup;

    println!("DESTRUCTURE: {}, {}", channel, subcount);
    println!("SUBNOTATION: {}, {}", tup.0, tup.1);
    // throws error: println!("ERROR MEM SAFETY {}", tup.2);

    my_function(5, 6);

    if_else(5);
    inline_logic(7);
    infinite_loop();
    for_loop();
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("x: {}, y: {}", x, y);
    println!("x + y = {}", x + y);

    //explicit return return x + y;
    x + y //implied return
}

fn if_else(num: i32) {
    if num > 5 {
        //notice lack of parenthesis in if blocks
        false;
    } else {
        true;
    }
}

fn inline_logic(num: i32) -> bool {
    let is_greater: bool = if num > 5 { true } else { false }; //again, notice lack of parenthesis

    return is_greater;
}

fn infinite_loop() {
    let mut counter = my_function(3, 5);
    loop {
        println!("Infinite Loop! COUNT = {}", counter);
        counter -= 1;

        if counter == 0 {
            break;
        }; //if break not present, loop is infinite
    }
}

fn for_loop() {
    let a = [1,2,4,14,15,16];
    for element in a.iter() {
        println!("{}", element);
    }

    for element2 in 0..my_function(7, 1) {
        println!("rnge from 0 to {} not inclusive: {}", my_function(7, 1), element2);
    }
}

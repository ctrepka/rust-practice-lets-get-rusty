struct User {
    username: String,
    //email: String,
    //sign_in_count: u64,
    //active: bool,
}

//struct Color(i32,i32,i32);

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct StartStop {
    p1: Point,
    p2: Point,
}

impl StartStop {
    pub fn create_rectangle(&self) -> Rectangle {
        let bl_x = self.p1.x;
        let bl_y = self.p2.y;
        let tr_x = self.p2.x;
        let tr_y: i32 = self.p1.y;

        let bl: Point = Point { x: bl_x, y: bl_y };

        let tr: Point = Point { x: tr_x, y: tr_y };

        return Rectangle {
            tl: self.p1,
            tr: tr,
            bl: bl,
            br: self.p2,
        };
    }
}

#[derive(Copy, Clone, Debug)]
struct Rectangle {
    tl: Point,
    tr: Point,
    bl: Point,
    br: Point,
}

impl Rectangle {
    pub fn new(ss: &StartStop) -> Self {
        let bl_x = ss.p1.x;
        let bl_y = ss.p2.y;
        let tr_x = ss.p2.x;
        let tr_y: i32 = ss.p1.y;

        let bl: Point = Point { x: bl_x, y: bl_y };

        let tr: Point = Point { x: tr_x, y: tr_y };

        return Self {
            tl: ss.p1,
            tr: tr,
            bl: bl,
            br: ss.p2,
        };
    }

    /*     pub fn update(tl: i32, tr: i32, bl: i32, br: i32) -> Self {

    } */

    pub fn area(&self) -> i32 {
        let area = get_dist(self.tr, self.tl) * get_dist(self.br, self.tr);

        area as i32
    }
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        //email: String::from("test@gmail.com"),
        username: String::from("test"),
        //active: true,
        //sign_in_count: 1
    };

    //let name = user1.username;
    user1.username = String::from("test1");

    let user2 = build_user(
        String::from("user2@gmail.com"), /*String::from("user2")*/
    );

    let user3 = User {
        //email: String::from("user3@gmail.com"),
        username: String::from("user3"),
        ..user2
    };

    println!("{} {} {}", user1.username, user2.username, user3.username);

    let rect1 = Rectangle::new(&StartStop {
        p1: Point { x: 1, y: 4 },
        p2: Point { x: 3, y: 1 },
    });

    println!("{:#?}", rect1);
    println!("{:#?}", rect1);

    println!("AREA IS {}", rect1.area());
    println!("AREA IS {}", rect1.area());

    let line: StartStop = StartStop {
        p1: Point { x: 1, y: 2 },
        p2: Point { x: 3, y: 4 },
    };

    let rect2 = line.create_rectangle();

    println!("{:#?}", rect2);
    println!("{}", rect2.area());
}

fn build_user(/* email: String ,*/ username: String) -> User {
    User {
        //email,
        username,
        //active: true,
        //sign_in_count: 1,
    }
}

fn get_dist(p1: Point, p2: Point) -> i32 {
    let dist = (((p2.x - p1.x).pow(2) as f32) + ((p2.y - p1.y).pow(2) as f32)).sqrt() as f32;

    dist as i32
}

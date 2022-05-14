// fn main() {
// // this is valid, and is called variable shadowing
// let x = 4;
// let x = "six";

// // cant use const for return statements of functions
// const SUB_COUNT: u32 = 100_000;

// // tuple
// let tup = ("yo yo", 100);

// let (text, number) = tup;

// let text = tup.0;
// let number = tup.1;

// let error_codes = [400, 404, 500];
// let not_found = error_codes[1];
// let sum = my_func(10, 20);
// println!("{}", sum);

// Control flow
// one way is if, else if, else

// let condition = true;
// let number = if condition { 5 } else { 6 };

// // loops
// let mut counter = 0;
// let result = loop {
//     counter += 1;
//     if counter == 10 {
//         break counter;
//     }
//     println!("{}", counter);
// };
// println!("result is: {}", result);

// while counter != 0 {
//     counter -= 1;
//     println!("{}", counter);
// }
// println!("LIFT OFF");

// let a = [10, 20, 30, 40, 50];

// for element in a.iter() {
//     println!("{}", element)
// }

// for number in 1..10 {
//     println!("{}", number)
// }
// }

// fn my_func(x: i32, y: i32) -> i32 {
//     println!("{} + {} = ", x, y);
//     // last line in function is return always, and we dont need to add semicolons on return statements
//     x + y
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main(){

// let mut user1 = User {
//     email: String::from("prhyme@gmail.com"),
//     username: String::from("prhyme"),
//     sign_in_count: 1,
//     active: true,
// };

// let name = user1.username;
// user1.username = String::from("prhyme89");

// let user2 = build_user(String::from("prhyme89@prhyme.com"), String::from("prhyme"));

// let user3 = User {
//     email: String::from("email@email.com"),
//     username: String::from("email"),
//     ..user2
// };

// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // impl consists of methods on struct
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, rect: &Rectangle) -> bool {
//         self.width > rect.width && self.height > rect.height
//     }
// }

// // we can have as many impl blocks for the same struct
// impl Rectangle {
//     // associative functions (not tied to the struct) dont take in &self keyword
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 20,
//         height: 40,
//     };

//     let rect2 = Rectangle {
//         width: 40,
//         height: 40,
//     };

//     let rect3 = Rectangle::square(25);
//     println!("{:#?}", rect3);

//     println!("{:#?}", rect1);
//     println!("{}", rect1.area());
//     println!("{}", rect1.can_hold(&rect2));
// }

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn some_function() {
//         println!("Lez go")
//     }
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

//     // enum Option<T> {
//     //     Some(T),
//     //     None,
//     // }

//     // x can either be 5 or null
//     // we cant add optional type with integer type
//     let x = 5;
//     let y = Some(5);
//     // let y = None;

//     let sum = x + y.unwrap_or(0);
// }

// #[derive(Debug)]
// enum UsState {
//     Alaska,
//     Alabama,
//     // ...
// }

// enum Coin {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter(UsState),
// }

// fn match_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickle => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("Quarter from {:?} state", state);
//             25
//         }
//     }
// }

// fn main() {
//     let amount = match_in_cents(Coin::Quarter(UsState::Alabama));
// }

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

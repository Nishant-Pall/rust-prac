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

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", six);
//     println!("{:?}", none);

//     let some_value = Some(3);

//     if let Some(3) = some_value {
//         println!("Three")
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//         // if pattern doesnt match above two then execute this one
//         _ => None,
//     }
// }

// fn main() {
//     // let a = [1, 2, 3];
//     // let mut v: Vec<i32> = Vec::new();
//     // v.push(1);
//     // v.push(2);
//     // v.push(3);

//     // calling vec macro to initialize vector
//     let v2 = vec![1, 2, 3, 4, 5];

//     // we can use out of bounds index, and it'll give runtime error not compile time
//     // to avoid that we use get method
//     let third = &v2[2];

//     match v2.get(2) {
//         Some(third) => println!("Third element is: {}", third),
//         None => println!("There is no third element"),
//     }

//     let mut v = vec![1, 2, 3, 4, 5];

//     for i in &mut v {
//         *i += 50;
//         // println!("{}", i)
//     }

//     for i in &mut v {
//         println!("{}", i)
//     }
// }

// fn main() {
//     enum SpreadSheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadSheetCell::Int(3),
//         SpreadSheetCell::Float(420.69),
//         SpreadSheetCell::Text(String::from("Blue")),
//     ];

//     match &row[0] {
//         SpreadSheetCell::Int(i) => println!("{}", i),
//         _ => println!("Not an integer"),
//     }
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World");

//     // let s3 = s1 + &s2;
//     // format! macro doesnt take ownerships
//     let s3 = format!("{}{}", s1, s2);
//     println!("{}", s3);

//     let hello = String::from("Hello");

//     for c in hello.chars() {
//         println!("{}", c)
//     }

//     for c in hello.bytes() {
//         println!("{}", c)
//     }
// }

use std::{collections::HashMap, hash::Hash};

fn main() {
    let yellow = String::from("Yellow");
    let blue = String::from("Blue");

    let mut scores = HashMap::new();

    scores.insert(blue, 100);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{}", score)
}

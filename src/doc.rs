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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

// Control flow
// one way is if, else if, else

// let condition = true;
// let number = if condition { 5 } else { 6 };

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

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

// ****************************************************************************************************************************
// ****************************************************************************************************************************

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World");

//     // let s3 = s1 + &s2;
//     // format! macro doesnt take ownerships
// 	   let s3 = format!("{}{}", s1, s2);
//     println!("{}", s3);

//     let hello = String::from("Hello");

//     for c in hello.chars() {
//         println!("{}", c)
//     }

//     for c in hello.bytes() {
//         println!("{}", c)
//     }
// }

// ****************************************************************************************************************************
// ****************************************************************************************************************************

// use std::{collections::HashMap, hash::Hash};

// fn main() {
//     let yellow = String::from("Yellow");
//     let blue = String::from("Blue");

//     let mut scores = HashMap::new();

//     scores.insert(blue, 100);
//     scores.insert(yellow, 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);

//     let text = "Hello world wonderful world";
//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         // if word doesnt exist, it'll enter the word into the map and default the value to 0
//         // or it'll get the mutable reference onto which we can perform operations
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

// ****************************************************************************************************************************
// ****************************************************************************************************************************

// use std::fs::{self, File};
// use std::io;
// use std::io::ErrorKind;
// use std::io::Read;
// fn main() {
//     // panic!("Crash and burn")

//     // enum Result<T, E> {
//     //     Ok(T),
//     //     Err(E),
//     // }

//     // let f = File::open("hello.txt").expect("Failed to open");

//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating file:{}", e),
//     //         },
//     //         other_error => panic!("Problem opening the file: {}", other_error),
//     //     },
//     // };

//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("Error opening file: {}", error),
//     // };
// }

// // ERROR PROPORGATION
// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut s = String::new();
//     // File::open("hello.txt")?.read_to_string(&mut s)?;
//     // Ok(s);

//     // let mut f = match f {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => Err(e),
//     // };
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let number_list = vec![10, 20, 30, 40000, 505];

//     let largest_number = get_largest(number_list);

//     let char_list = vec!['y', 'a', 'm', 'c'];
//     let largest_char = get_largest(char_list);

//     println!("{}, {}", largest_number, largest_char);
// }

// fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
//     let mut largest = list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: 5.0, y: 10.0 };

//     let p3 = Point {
//         x: "Hello",
//         y: "World",
//     };

//     let p4 = p1.mixup(p3);

//     println!("p4.x is: {}, p4.y is: {}", p4.x, p4.y)
// }

// TRAITS
// use std::fmt::{Debug, Display};

// pub struct NewsArticle {
//     pub headline: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.headline, self.author)
//     }
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }

// pub struct Tweet {
//     pub content: String,
//     pub username: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
//     fn summarize_author(&self) -> String {
//         format!("{}", self.username)
//     }
// }

// pub trait Summary {
//     fn summarize_author(&self) -> String;
//     fn summarize(&self) -> String {
//         // default implementation
//         String::from("(Read more...)")
//     }
// }

// // pub fn notify(item: &impl Summary) {
// //     println!("Breaking news!: {}", item.summarize())
// // }

// // pub fn notify<T: Summary>(item: &T) {
// //     println!("Breaking news!: {}", item.summarize())
// // }

// // pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
// //     // ...
// // }

// // pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
// //     //...
// // }

// pub fn some_function<T, U>(t: &T, u: &U)
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     // ...
// }

// fn main() {
//     let tweet = Tweet {
//         content: String::from("Hello World"),
//         username: String::from("@johndoe"),
//         reply: false,
//         retweet: false,
//     };

//     let article = NewsArticle {
//         headline: String::from("The sky is falling"),
//         content: String::from("Yes, the sky is falling"),
//         author: String::from("John Doe"),
//     };

//     // notify(&article);

//     // println!("Tweet Summary: {}", tweet.summarize());
//     // println!("News Summary: {}", article.summarize());
//     // println!("{}", article.summarize_author());
//     // println!("{}", tweet.summarize_author());
// }

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: PartialOrd + Display> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest number is: {}", self.x);
//         } else {
//             println!("The largest number is: {}", self.y);
//         }
//     }
// }

// fn main() {}

// ---------------------------------------------------------------------------
// GENERIC LIFETIMES
// ---------------------------------------------------------------------------

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");

//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longer string is: {}", result)
// }

// // &i32 -------> a reference
// // &'a i32 ----> a reference with an explicit lifetime

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// ===========================================================
// Struct lifetimes
// ===========================================================

// // struct cannot outlived the reference passed into part variable
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find message");

//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// THE RULES OF LIFETIMES IN PARAMETERS

// Each parameter that is a reference gets its own lifetime parameter

// If there is only one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

// If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned
// to all other output lifetime parameters

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("The announcement is: {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

// TESTING

// struct Rectangle {
//     width: i32,
//     height: i32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 7,
//             height: 8,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller), "Larger cannot hold smaller");
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 7,
//             height: 8,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         assert!(!smaller.can_hold(&larger));
//     }

//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 3 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus three does not equal four"))
//         }
//     }
// }

// ***********************************************************************************
// ***********************************************************************************

// use std::thread;
// use std::time::Duration;

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculating slowly....");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let mut expensive_closure = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure.value(intensity));
//         println!("Next, do {} situps!", expensive_closure.value(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure.value(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let expensive_closure = |s| s;

//     let s = expensive_closure(String::from("Hello"));
//     // cant use integer as compiler has annotated the closure according to its calling from above expression
//     // let n = expensive_closure(5);
//     let intensity = 7;
//     let random_number = 10;

//     generate_workout(intensity, random_number);
// }

// ***********************************************************************************
// ***********************************************************************************

// #[derive(PartialEq, Debug)]

// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_by_size() {
//         let shoes = vec![
//             Shoe {
//                 size: 10,
//                 style: String::from("sneaker"),
//             },
//             Shoe {
//                 size: 13,
//                 style: String::from("sandal"),
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot"),
//             },
//         ];
//         let in_my_size = shoes_in_my_size(shoes, 10);

//         assert_eq!(
//             in_my_size,
//             vec![
//                 Shoe {
//                     size: 10,
//                     style: String::from("sneaker")
//                 },
//                 Shoe {
//                     size: 10,
//                     style: String::from("boot")
//                 },
//             ]
//         );
//     }
// }

// fn main() {
//     // let v1 = vec![1, 2, 3];

//     // let v1_iter = v1.iter();

//     // for iter in v1_iter {
//     //     println!("{}", iter);
//     // }

//     // let v1 = vec![1, 2, 3];
//     // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     // assert_eq!(v2, vec![2, 3, 4]);
// }

// ****************************************************************************************
// ****************************************************************************************

// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// #[test]
// fn calling_next_iterator() {
//     let mut counter = Counter::new();

//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), None);
// }

// ****************************************************************************************
// ****************************************************************************************

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_prac::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

// ****************************************************************************************
// ****************************************************************************************

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     // let b = Box::new(5);
//     // println!("{}", b);

//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// ****************************************************************************************
// ****************************************************************************************

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);

//     let m = MyBox::new(String::from("rust"));

//     hello(&m);
//     // &MyBox<String> -> &String -> &str
//     // The automatic deref coercion feature of rust converts the type to the function signature,
//     // dereferencing &String gives us &str(string slice)

//     // If rust didnt have this feature we would have to do
//     hello(&(*m)[..])
// }

// fn hello(name: &str) {
//     println!("{}", name)
// }

// ****************************************************************************************
// ****************************************************************************************

// use std::rc::Rc;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

//     println!("{}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("{}", Rc::strong_count(&a));
//     }

//     println!("{}", Rc::strong_count(&a));
// }

// ****************************************************************************************
// ****************************************************************************************

// use std::{thread, time::Duration};

// fn main() {
//     let handler = thread::spawn(|| {
//         for i in 1..10 {
//             println!("The number from spawned thread: {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // this will block the main thread from finishing until handler gets finished
//     // when main thread finishes, all the other threads finish
//     // the position where join is called can effect the running of the process
//     handler.join().unwrap();

//     for i in 1..5 {
//         println!("The number from main thread: {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     let v = vec![1, 2, 3];

//     // closure may outlive the current function, but it borrows `v`, which is owned by the current function
//     // may outlive borrowed value `v`
//     // hence we move the ownership of v to the closure using 'move'
//     let handler = thread::spawn(move || {
//         println!("{:?}", v);
//     });

//     handler.join().unwrap();
// }

// ****************************************************************************************
// ****************************************************************************************

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     // cloning tx for the second thread to create multiple transmitters
//     let tx2 = tx.clone();

//     // create another thread that will send a message to main thread
//     thread::spawn(move || {
//         // let msg = String::from("hi");
//         // tx.send(msg).unwrap();

//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the "),
//             String::from("thread1"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     // create another thread that will send a message to main thread
//     thread::spawn(move || {
//         // let msg = String::from("hi");
//         // tx.send(msg).unwrap();

//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx2.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//     // .rec() will block the main channel until a message is received
//     // .try_recv() will not block the channel and will return a result type immediately
//     // let received = rx.recv().unwrap();

//     for received in rx {
//         println!("Got :{}", received);
//     }
// }

// ****************************************************************************************
// ****************************************************************************************

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // let m = Mutex::new(5);
//     // {
//     //     // locking the m variable
//     //     let mut num = m.lock().unwrap();
//     //     *num = 6;
//     // }
//     // println!("{:?}", m);

//     let counter = Arc::new(Mutex::new(0));

//     let mut handler = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });

//         handler.push(handle);
//     }

//     // block main thread until handle is finished
//     for handle in handler {
//         handle.join().unwrap()
//     }

//     println!("{}", counter.lock().unwrap())
// }

// ****************************************************************************************
// ****************************************************************************************
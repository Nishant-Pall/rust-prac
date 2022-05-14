fn main() {
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
    let sum = my_func(10, 20);
    println!("{}", sum);

    // Control flow
    // one way is if, else if, else

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
        println!("{}", counter);
    };
    println!("result is: {}", result);

    while counter != 0 {
        counter -= 1;
        println!("{}", counter);
    }
    println!("LIFT OFF");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element)
    }

    for number in 1..10 {
        println!("{}", number)
    }
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("{} + {} = ", x, y);
    // last line in function is return always, and we dont need to add semicolons on return statements
    x + y
}

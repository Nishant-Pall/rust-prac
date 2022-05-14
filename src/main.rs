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
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("{} + {} = ", x, y);
    // last line in function is return always, and we dont need to add semicolons on return statements
    x + y
}

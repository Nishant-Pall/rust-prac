use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("rust"));

    hello(&m);
    // &MyBox<String> -> &String -> &str
    // The automatic deref coercion feature of rust converts the type to the function signature,
    // dereferencing &String gives us &str(string slice)

    // If rust didnt have this feature we would have to do
    hello(&(*m)[..])
}

fn hello(name: &str) {
    println!("{}", name)
}

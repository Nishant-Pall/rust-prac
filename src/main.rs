fn main() {
    let mut num = 5;

    // raw pointers do not follow any ownership rules
    // we cant dereference raw pointers unless we use
    // unsafe keyword
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }

    unsafe fn dangerous() {}

    // unsafe functions can only be called inside unsafe blocks or functions
    unsafe {
        dangerous();
    }
}

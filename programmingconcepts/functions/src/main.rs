fn main() {
    println!("Hello, world!");
    another_function();

    let five = five();
    let fifty = fifty(10);

    println!("{five}");
    println!("{fifty}");
}

//normal function
fn another_function() {
    println!("Another function")
}

//function with parameters
fn another_functionb(x: i32) {
    println!("Another function")
}

//return value function
fn five() -> i32 {
    5
}

fn fifty(number: i32) -> i32 {
    5 * number
}

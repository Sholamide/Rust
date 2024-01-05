//mutable strings
fn main() {
    let mut x = 20;
    println!("{}", x);

    x = 30;
    println!("{}", x);

    //constants
    const CONSTANT: u32 = 60 * 60 * 2;
    println!("{}", CONSTANT);

    //shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

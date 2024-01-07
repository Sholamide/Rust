fn main() {
    let number = 3;
    let number_string = if number > 3 {
        "greather than 3."
    } else {
        "Less than three."
    };

    if number > 1 {
        println!("Hello, number is greater than 1!");
    } else {
        println!("Number is less than 1!");
    }

    println!("Number string is: {number_string}");

    // breakable loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //for loop rever range
    for number in (1..5).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF 1-4!!!");
}

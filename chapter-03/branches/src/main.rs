fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // these are called arms blocks of codes within each expression
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if is an expression why it can be used on the right side of a let

    println!("The value of number is: {number}");

    let mut count = 0;
    'counting_up: loop {
        // can label a loop and then break his specific loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // breaking out the current loop
            }
            if count == 2 {
                break 'counting_up; // breaking out the spcified loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        // simple for loop to go through an array
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        // going in reverse with a range it excludes the last number
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

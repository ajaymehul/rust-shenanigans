fn main() {
    let number: i32 = 5;

    if number < 5 {
        println!("{} number is less than 5", number);
    } else {
        println!("{} number is not less than 5", number);
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // For loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}



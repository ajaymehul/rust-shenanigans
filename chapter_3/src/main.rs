use std::io;

fn main() {
    // Mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // CONSTANT
    const I_CANNNOT_CHANGE: u32 = 2 * 4;
    println!("The value of I_CANNOT_CHANGE is {}", I_CANNNOT_CHANGE);

    // Parsing
    let parsed_number: u32 = "42".parse().expect("Not a number!");
    println!("The value of parsed_number is {}", parsed_number);

    // Tuples

    let tup = (500, 200, "What");
    let (x,y,z) = tup; // Some shadowing happening here lol
    println!("I am tuple restructured after destructuring: ({},{},{})", x,y, z);
    println!("I am tuple element accessed through indexing: {}", tup.0);
    
    // Arrays

    let a = [1,2,3,4,5];
    println!("I am second value of array a: {}", a[1]);
    let a: [i32; 5] = [1,2,3,4,5]; //type & size explicit
    println!("I am second value of array a: {}", a[1]);
    let a = [3; 5]; //repeat same value: [3; 5] = [3,3,3,3,3]
    println!("I am last value of array a: {}", a[4]);

    // Element Access Error
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

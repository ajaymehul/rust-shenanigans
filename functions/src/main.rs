fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(20, 'F');
    let x = 10;
    println!("I am {} + 1: {}",x, plus_one(x));
}

fn another_function(x: i32) {
    println!("I'm another function");
    println!("My parameter is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
// Rite of passage fibonacci application right here
fn main() {
    println!("Hello, world!");
    let x: u32 = 11;
    println!("Fibonacci({}) = {}", x, fibonacci(x));
}

fn fibonacci (x: u32) -> u32{
    let mut last_number = 0;
    let mut number = 1;
    if x == 0 {return 0;}
    else if x == 1 {return 1;}

    let mut counter = 1;
    loop {
        if counter == x {break number}
        let temp = number;
        number = number + last_number;
        last_number = temp;
        counter+=1;
    }
}

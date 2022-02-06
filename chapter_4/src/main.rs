
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("main() holds string: {}", s);
    println!("main() is giving away the string");
    s = takes_and_gives_ownership(s);
    println!("main() owns string: {}", s);
    println!("main() rides again!!");
}

fn takes_and_gives_ownership(s: String) -> String {
    println!("takes_and_gives_ownership() now owns string: {}:", s);
    println!("I was raised to be a respectable function. Give what thy took");
    return s;
}

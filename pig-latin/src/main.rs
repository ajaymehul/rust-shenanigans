fn main() {
    let words = vec!["First", "Second", "Alphabet"];
    for word in &words {
        let pig_latin = convert_to_pig_latin(word);
        println!("{} ---> {}", word, pig_latin);
    }
}


fn convert_to_pig_latin(s: &str) -> String {
    let s = String::from(s); //shadowing woo
    let mut pig_latin = String::new();


    let mut ignore_flag = 0; // don't push into pig_latin while this is 0
    let mut first_char = ' '; //temporary. This is bad code, but rust is hard
    for c in s.chars() {
        if ignore_flag == 0 {
            ignore_flag = 1;
            first_char = c;
        }
        else {
            pig_latin.push(c);
        }
    }
    pig_latin.push('-');
    pig_latin.push(first_char);
    pig_latin.push_str("ay");

    return pig_latin
}


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


    let mut first_flag = 0; // first char flag when 0
    let mut first_char = None; //temporary. This is bad code, but rust is hard
    for c in s.chars() {
        if first_flag == 0 {
            first_flag = 1;
            if ['a','e','i','o','u','A','E', 'I','O','U'].contains(&c){
                pig_latin.push(c);
            } else {
                first_char = Some(c);
            }
            
        }
        else {
            pig_latin.push(c);
        }
    }
    pig_latin.push('-');
    match first_char {
        Some(c) => {pig_latin.push(c);}
        None => {pig_latin.push('h')}
    }
    pig_latin.push_str("ay");

    return pig_latin
}
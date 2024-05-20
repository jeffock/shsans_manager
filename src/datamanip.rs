pub fn testing(s: &mut String) {
    println!("{}", s);
}

pub fn tokenizer(s: &mut String) -> Vec<String> {

    // splits input String via whitespace and collects it as a Vec //
    return s.split_whitespace().map(String::from).collect();
}


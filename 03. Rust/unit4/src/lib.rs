pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("The string is plural.");
    } else {
        println!("The string is singular.");
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly");
}
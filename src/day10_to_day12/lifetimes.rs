pub fn examples() {
    println!("Lifetimes Examples:");
    
    let string1 = String::from("short");
    let string2 = String::from("longer");
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
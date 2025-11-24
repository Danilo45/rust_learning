fn main() {
    let s = String::from("hello");
    let len = calculate_length(s.clone());
    println!("The length of '{}' is {}", s, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

// The problem in this exercise was taking ownership from s by calculate_length. After that, s does not exist in main, so in order to fix this we need to use .clone()

/* second way to fix this would be using reference like this : 
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}*/
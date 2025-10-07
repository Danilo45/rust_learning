fn main() {
    // Immutable variable (can't be changed)
    let age = 24;
    println!("My age is: {}", age);

    // Mutable variable (can be changed)
    let mut favorite_number = 14;
    println!("My favorite number is: {}", favorite_number);

    favorite_number = 42;
    println!("Now my favorite number is: {}", favorite_number);

    // Shadowing: reuse the same variable name but change its type
    let text = "Danilo";
    let text = text.len(); 
    println!("The length of the word is: {}", text);
}

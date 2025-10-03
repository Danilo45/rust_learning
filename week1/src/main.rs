fn main() {
    println!("Hello, World from Rust!!!");
    println!("`println!` without `!` will cause an error, because `!` means it is a macro.");
    println!("The function name is case-sensitive, so if I rename `main` to `Main` it will cause an error.");
    println!("Rust is a memory-safe programming language. I want to learn it because it feels safer than C and C++ in terms of memory.");
    println!("It doesn't let you make common mistakes like null or dangling pointers.");
    print!("This is different macro, it does not add new line at the end.");
    print!("\n");
    print!("Hello ");
    eprintln!("An error occurred: invalid input");
    let name = "Danilo";
    let age = 24;
    let message = format!("My name is {} and I am {} yo.", name, age);
    println!("{}", message);
    let msg2 = format!(
        "We can do it \
        this way also:
        name: {my_name},
        age: {my_age}",
        my_age=age,
        my_name= name
    );
    println!("{}", msg2);
}

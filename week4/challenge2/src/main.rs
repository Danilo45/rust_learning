// Takes ownership and returns ownership
fn take_and_return(mut s: String) -> String {
    // Add " modified" to the string
    // Your code here
    s.push_str(" modified");
    s
}

// Borrows immutably and returns new String
fn borrow_and_create(s: &String) -> String {
    // Return uppercase version
    // Your code here
    s.to_uppercase()
}

// Borrows mutably
fn borrow_and_modify(s: &mut String) {
    // Add " modified" to the string
    // Your code here
    s.push_str(" modified");
}

fn main() {
    // Test all three functions
    let s1 = String::from("test");
    let s2 = take_and_return(s1);
    println!("{}", s2);
    
    let s3 = String::from("test");
    let s4 = borrow_and_create(&s3);
    println!("{} and {}", s3, s4);
    
    let mut s5 = String::from("test");
    borrow_and_modify(&mut s5);
    println!("{}", s5);

    println!(
    "take_and_return: takes ownership, modifies the value, and returns the owned String.
borrow_and_create: borrows immutably (&), does NOT modify the original, and returns a NEW String.
borrow_and_modify: borrows mutably (&mut), modifies the original in-place, and does NOT return a new value."
    );

}
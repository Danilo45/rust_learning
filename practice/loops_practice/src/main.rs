// Check if a given char array palindrome or not.

fn is_palindrome(char_array: &[char]) -> bool {
    let s: String = char_array.iter().collect();
    print!("\'{}\' ", s);
    let n = char_array.len();
    for i in 0..=(n/2){
        if char_array[i] != char_array[n-1-i]{
            return false;
        }      
    }
    true
}
 
fn main() {
    let char_array_1 = ['r', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_1) {
        println!("is a palindrome.");
    } else {
        println!("is not a palindrome");
    }
 
    let char_array_2 = ['b', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_2) {
        println!("is a palindrome.");
    } else {
        println!("is not a palindrome");
    }

    let char_array_3 = ['a', 'n', 'n', 'a'];
    if is_palindrome(&char_array_3) {
        println!("is a palindrome.");
    } else {
        println!("is not a palindrome");
    }
 
    let char_array_4 = ['r', 'o', 't', 'a', 't', 'o', 'r'];
    if is_palindrome(&char_array_4) {
        println!("is a palindrome.");
    } else {
        println!("is not a palindrome");
    }
    
    let char_array_5 = ['d', 'a', 'n', 'i', 'l', 'o'];
    if is_palindrome(&char_array_5) {
        println!("is a palindrome.");
    } else {
        println!("is not a palindrome");
    }
}

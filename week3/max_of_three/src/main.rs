fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    let mut max = a; // start with assumption that the first number is the max

    // if we find a value greater than the current max, it becomes the new max
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }

    max 
}

fn main() {
    println!("Max of (3, 7, 5) = {}", max_of_three(3, 7, 5));
    println!("Max of (10, 2, 8) = {}", max_of_three(10, 2, 8));
    println!("Max of (-1, -5, -3) = {}", max_of_three(-1, -5, -3));
}
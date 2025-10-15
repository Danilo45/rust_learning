fn is_even(n: i32) -> bool {
    //statement would be: return n%2==0;
    n % 2 == 0 //expression (same funcionality, smoother code, idiomatic Rust)
}

fn main() {
    for i in 1..=10 {
        println!("{} is even? {}", i, is_even(i));
    }
}
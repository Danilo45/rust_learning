fn main() {
    let numbers: (i32, u8, f64) = (10, 2, 30.7);

    let (a, b, c) = numbers;
     
    let sum = a as f64 + b as f64 + c;
    let average = sum / 3.0; 
    let product = a as f64 * b as f64 * c;

    println!("Numbers: ({}, {}, {})", a, b, c);
    println!("Sum: {}", sum);
    println!("Average: {}", average);
    println!("Product: {}", product);
}

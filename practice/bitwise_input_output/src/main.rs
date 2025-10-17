/* User will enter number in decimal format, and the index of the bit they want to check
Program will display if bit is 1 or 0 */
use std::io;
use std::io::Write;

fn bitcheck(number: u32, index: u32) -> bool{
    number & (1 << index) != 0 
}

fn main() {
    print!("Enter number in decimal format: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().parse::<u32>().expect("Please enter a valid number");
    print!("Enter bit number you want to check: (0-31) ");
    io::stdout().flush().unwrap();
    let mut input_index = String::new();
    io::stdin().read_line(&mut input_index).expect("Failed to read line");
    let input_index = input_index.trim().parse::<u32>().expect("Enter a valid bit number");
    let is_true = bitcheck(input, input_index);
    if is_true {
        println!("Number {}\nBinary {:032b}\nBIT {}: 1", input, input, input_index);
    }else {
        println!("Number {}\nBinary {:032b}\nBIT {}: 0", input, input, input_index);
    }    
}

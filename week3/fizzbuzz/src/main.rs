//Pattern matching
fn main() {
    for i in 1..=100{
        match(i % 3, i % 5){ //remainders when dividing by 3 and 5, respectively
            (0,0) => println!("FizzBuzz"),  
            (0,_) => println!("Fizz"),
            (_,0) => println!("Buzz"),
            _ => println!("{}", i),            
        }
    }
}
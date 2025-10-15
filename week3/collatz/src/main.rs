fn collatz(mut n: u32) -> u32{
    let mut steps = 0;
    print!("For n={}, the sequence is: {}", n, n); 
    while n != 1{
        if n % 2 == 0{
            n /= 2; 
        }else{
            n = 3*n + 1;
        }
        print!(", {}", n);
        steps += 1;
    }
    steps
}

fn display_collatz(num_of_steps: u32){
    println!("({} steps)", num_of_steps);
}

fn main() {
    display_collatz(collatz(6));
    display_collatz(collatz(9));
    display_collatz(collatz(3));
    display_collatz(collatz(2));
}

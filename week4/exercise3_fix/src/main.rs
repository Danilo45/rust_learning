// Problem: We cannot have &s (immutable) and &mut s (mutable) at the same time
fn fix1() {
    //There can be more than 1 immutable references of s.
    let s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("{}, {}, and {}", r1, r2, r3);
}

fn fix2() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    print!("{}, {}", r1, r2); // last use of r1 and r2! After this, they are no longer active borrows
    let r3 = &mut s; //now since s is not borrowed immbutably, we can create a mutable reference.
    print!(", and {}",  r3);
}
fn main() {
    fix1();
    fix2();
}
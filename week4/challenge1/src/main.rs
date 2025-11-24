/*
You may have many immutable references OR exactly one mutable reference, but not both at the same time.
*/
fn process_strings() -> String {
    let mut s1 = String::from("hello");
    let s2 = String::from(" world");
    
    let r1 = &mut s1;
    r1.push_str(&s2);
   /*Here, you cannot have immutable and muitable references to s1 active at the same time
    let r2 = &s1;
    let r3 = &mut s1;
    We dont need r2 here. 
     */ 
    //fix:
    let r3 = &mut s1;
    r3.push_str("!");
    
    println!("{}", r3);  
    /* Returning r3 is also mistake, because it is reference to String. 
    After ending of function, s1 will drop, and r3 would point to nothing!
    That is a dangling pointer, and RUST can prevent it by not compiling the code. 
    So in orger to fix this, we can return s1 String, not reference that points to this string! 
     */
    s1
}

fn main() {

    let result = process_strings();
    println!("{}", result);
}
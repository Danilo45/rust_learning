fn function1() {
    //this fn will compile, because r is just immutable reference to s.
    //we have only immutable reference &s, and there is no conflict. 
    //This would also work if we had more than 1 immutable references
    let s = String::from("hello");
    let r = &s;
    println!("{}", s);
    println!("{}", r);
}
fn function2() {
    //this fn will NOT compile, because
    //s cannot have an immutable and mutable reference active at the same time.
    //So, we can make many immutable references to variable, and we can make 1 mutable reference to variable,
    //but we CAN NOT make mut and immutable together !!! 
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}", r1);
}
fn function3() {
    //this fn will NOT compile!
    //we can make many immutable references to variable, and we can make 1 mutable reference to variable,
    //we CAN NOT make more than 1 active mutable reference 
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}", r1);
}

fn main(){
    println!("In order to prevent data races, there is rule in RUST about references:");
    println!("You may have many immutable references OR exactly one mutable reference, but not both at the same time.");
    function1();
}
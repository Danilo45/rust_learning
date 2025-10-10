fn main() {
    // OWNERSHIP
    println!("\n{:-<25}", "");
    println!("Ownership");
    println!("{:-<25}", "");
    let my_tuple = (String::from("dani"), 9, 0.9);
    let my_tuple_2 = (String::from("lo"), 91, 1.9);
    let my_tuple_3 = (String::from("tamindzija"), 112, 14.10);
    let b = my_tuple;
    println!("{:?}", b);
    //println!("{:?}", my_tuple); not valid, since ownership was moved to b !
    let c = my_tuple_2.clone();
    println!("{:?}", c);
    println!("{:?}", my_tuple_2); //this is valid, because it made a deep copy without taking ownership!
    let d = &(my_tuple_3);
    println!("{:?}", d);
    println!("{:?}", my_tuple_3); //this is valid because d didn't move ownership because we borrowed my tuple3 by reference

    
    
    // BORROWING
    println!("\n{:-<25}", "");
    println!("Borrowing");
    println!("{:-<25}", "");
    let name = String::from("Dani");
    let n1 = &name;
    let n2 = &name;
    let n3 = &name; //multiple immutable borrows are allowed, but they are not mutable
    // n3.push_str(" Tamindzija"); this is not valid because we used &, and not & mut! 
    // let n4 = &mut name;  this is not valid because immutable borrows are still active
    // n4.push_str(" Tamindzija");
    println!("{}, {}, {}", n1, n2, n3);

    let mut first_name = String::from("Danilo"); //we must declare this as mut if we want to use &mut later
    let f1 = &mut first_name;
    f1.push_str(" Tamindzija"); //now this is valid since we are using mutable var
    // let f2 = &mut first_name; this is not valid because you can do only one mutable reference at a time! 
    // f2.push_str(", Novelic"); 
    println!("{}", f1);
    println!("\n{:-<25}", "");
    println!("Scope lifetime");
    println!("{:-<25}", "");
    {
        let scope_name = String::from("Daniloo");
        println!("Inside block: {}", scope_name); //valid - var is living inside scope
    } // scope_namegoes out of scope here Rust calls drop 

    // println!("{}", scope_name); not valid, since it is not avaliable in this scope

    {
        let mut s = String::from("Danilo");

        let r1 = &s;
        println!("r1: {}", r1); // This is last time r1 is used, so it will not wait till the end of a scope to drop it

        //borrow checker know that after last line, r1 is not borrowed anymore:  
        let r2 = &mut s; // we can take &mut now
        r2.push_str(" Tamindzija");
        println!("r2: {}", r2);
    } // r2 and s dropped here 



    // COPY VS NON COPY TYPES
    println!("\n{:-<25}", "");
    println!("Copy vs non copy types");
    println!("{:-<25}", "");
    let x = 5; // i32 implements Copy
    let y = x; // copy, not move
    println!("x = {}, y = {}", x, y); // still valid

    let s1 = String::from("Rust"); // String does NOT implement Copy
    let s2 = s1; // move
    // println!("{}", s1); // invalid, ownership moved
    println!("s2 = {}", s2);


    // MEMORY SAFETY IN RUST: 
    // dangling reference in rust
    println!("\n{:-<25}", "");
    println!("Memory-safety in rust");
    println!("{:-<25}", "");
    println!("\n- Dangling reference in RUST:");
    let r;
    {
        let s = String::from("Novelic");
        r = &s; //s will be dropped here
        println!("{}", r); // must read r once 
    }
    //println!("{}", r);  would be a dangling pointer, but rust doesn't allow this  


    // no null pointer dereference  

    println!("\n- No null pointer dereference in RUST:");

    // rust doesn't have NULL pointers, instead rust uses Option<T>
    let maybe_number: Option<i32> = Some(10);
    let maybe_none: Option<i32> = None;
    match maybe_number {
        Some(value) => println!("Number is {}", value),
        None => println!("No number found"),
    }
    if let Some(value) = maybe_none {
        println!("Number is {}", value);
    } else {
        println!("Value is None, cannot dereference!");
    }

    //no double free error
    println!("\n- Double free prevention in RUST:");
    {
        let s1 = String::from("Learning rust"); //allocate heap memory
        let s2 = s1; // ownership moved — s1 no longer exist 

        // println!("{}", s1); // not valid, because value moved

        println!("{}", s2); // valid — s2 is owning data now

        // when s2 goes out of scope, memory will be deallocated exactly once, rust will take care of that
    }
    // no buffer overflow
    println!("\n- Buffer overflow prevention in RUST:");
    let numbers = [10, 20, 30];
    println!("Element 0: {}", numbers[0]);
    println!("Element 2: {}", numbers[2]);

    
    // println!("Element 5: {}", numbers[5]); not valid, this would cause error immediately !   

    // Rust alternative:
    if let Some(value) = numbers.get(5) {
        println!("Value at index 5: {}", value);
    } else {
        println!("Index 5 is out of bounds — access prevented!");
    }



    // no memory leaks
    println!("\n- Memory leaks (mostly avoided) in RUST:");

    // Memory is automatically freed when variables go out of scope.
    // So, we can't forget to free the memoty, rust will take care of that

    {
        let s = String::from("Rust.");
        println!("{}", s);
    } // s goes out of scope here, drop() called automatically, memory freed 

    //no uninitialized memory access 
    println!("\n- No uninitialized memory access in RUST:");

    let _uninitialized_tuple: (i32, f64); 
    //println!("{:?}", uninitialized_tuple); // not valid, this will cause compile-time error
    {
        let initialized_tuple: (i32, f64) = (10, 3.14);
        println!("initialized_tuple: {:?}", initialized_tuple);
    }
    //no type confusion
    println!("\n- No type confusion:");
    {
        let t1 = (1, 2.0);
        // let t2: (f32, f32) = t1; // not valid, mismatched types  
        // println!("{:?}", t2);
        println!("{:?}", t1);

    }

    {
        let t1 = (1, 2.0);
        let t2 = (t1.0 as f32, t1.1); 
        println!("{:?}", t2);
    }
}

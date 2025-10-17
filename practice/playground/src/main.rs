fn convert_kg_to_grams(in_kg: f64) -> f64{ //use snake_case to write function names
    let grams = in_kg * ( 1000 as f64 );
    grams 
}

fn concatenate_strings(first: &str, second: &str) -> String{
    first.to_string() + second
}

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


    //week3
    println!("\n{:-<25}", "");
    println!("WEEK 3");
    println!("{:-<25}", "");
    //functions:
    println!("{}", convert_kg_to_grams(10 as f64));
    println!("{}",concatenate_strings("Danilo", " Tamindzija"));

    //Slice in rust is borrowed portion of array, vector or string
    let my_array: [i32; 4] = [1, 2, 3, 4];
    let slice1 = &my_array[1..3]; //from index 1 to index 3 = imtems 2 and 3 it is exclusive (..= for inclusive)

    //slice has informations about starting addr of slice and size of slice
    let slice2 = &my_array[..];
    let slice3 = &my_array[0..=1];
    println!("Slice1: {:?},\nSlice2: {:?},\nSlice3: {:?}.", slice1, slice2, slice3);
    let mut sum = 0;
    for i in slice1{     //slice1 type is &i32, so i is same type
        sum = sum + *i; // will work and without "*" - i32 + &i32 is valid in Rust
    }
    println!("Suma je {}", sum);
    //if we want to modify : 
    let mut my_array2 = [3, 3, 3, 3, 3];
    let mut_slice = &mut my_array2[1..];
    mut_slice[1] = 0;
    println!("Array after: {:?}", my_array2);

    //arrays
    //sort, reverse 
    {
        let mut my_arr = [3, 10, 20, 22, 13];
        let s = &mut my_arr;
        s.sort(); 
        s.reverse();
        println!("Array reverse sorted is : {:?}. Biggest is : {}", my_arr, my_arr[0]);
    }

    //concat
    {
        let arr1 = [1, 2, 3];
        let arr2 = [22, 31, 3];
        let arr3 = [arr1, arr2].concat();
        println!("{:?}+ {:?} = {:?}", arr1, arr2, arr3);
    }
    //split_at - returns a tuple
    {
        let big_arr = [2, 4, 5, 6, 7, 9, 10, 2];
        let (l, r) = big_arr.split_at(3);
        println!("{:?}", l);
        println!("{:?}", r);
    }

    //if else with expressions
    let age = 13;
    let msg = if age < 18{
        println!("This is statement!");
        "Not allowed to vote!" //expression
    }else {
        "Allowed to vote" //must be same type as if 
    };
    println!("{}", msg);

    // match as expression:
    // Check if any element in the array is negative
    {
        let array1 = [-5, 2, 3, 4];
            let is_invalid_array = match array1 {
                [n, _, _, _] | [_, n, _, _] | [_, _, n, _] | [_, _, _, n]if n < 0 =>{ 
                    true
                }, 
            _ => false
                 
        };
        if is_invalid_array {
            println!("invalid");
        }else {
            println!("not invalid");
        }

    }

    // matches macro, for true/false outcomes
    // matches!(expression, pattern);
    {
        let array2 = [1, -2, 3, 4];
        let invalid_array = matches!(array2, [n, _, _, _] | [_, n, _, _] | [_, _, n, _] | [_, _, _, n] if n < 0);
    
        if invalid_array {
            println!("invalid");
        }else {
            println!("not invalid");
        }
    }

    //if let  pattern matching 
    {
        let point = (4, 1);
        if let (0, 0) = point{
            println!("Point is at the beginning");
        } else if let (_, y @ 1..=4) = point{ // check if the second element of the point is in the range 1 to 4 (inclusive)
            println!("y= {} is in the range!", y); //we can use y as a local variable
        } else {
            println!("y = {} is out of the range", y);
        }
        
    }

    //input output

    use std::io;
    use std::io::Write;
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    print!("Before trim: {}", input);
    println!("After trim: {}", input.trim()); 
    print!("Enter some number: ");
    io::stdout().flush().unwrap();
    let mut input_number= String::new();
    io::stdin().read_line(&mut input_number).expect("failed to read line");
    let input_number = input_number.trim().parse::<u32>().expect("Please enter a valid number");
    let result = input_number + 10;
    println!("{} + 10 = {}", input_number, result);
}

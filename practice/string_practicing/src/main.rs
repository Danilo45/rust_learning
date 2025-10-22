fn main() {
    //String is mutable
    //different methods for string creation: 
    let mut my_string = String::new();
    my_string.push_str("Hello");
    let mut my_string2 = String::from("Hello");
    let my_string3 = "Hello".to_string(); 
    println!("These are different ways to create strings
    Created strings are : {}, {}, {}",  my_string, my_string2, my_string3);
    println!("String references contains infomation about:
    reference;
    length;
    capacity.");
    let str1 = &my_string[1..=3];
    println!("Type of slice is &[T],in this case &str.
    &str is not mutable, but we can read it: {}", str1);
    let str2 = "Hello!";
    println!("String literal is also slice ( &str ), we can read it: {}", str2);
    println!("str is not same as slice.");
    println!("We don't know clear boundaries of &str, because strings not only contain ascii chars (1 byte).");
    println!("String can contain also emoji, or some special UTF-8 characters that are more than 1 byte.");
    println!("So, we can't change length or characters of slice.");
    let str3 = &mut my_string2[1..4]; 
    str3.make_ascii_uppercase();
    //str3[1] = b'g';
    println!("Line : str3[1] = b'g' is not possible because, as we said, &str does not know clear boundaries. {}", str3);
    //slice of i32 numbers
    let mut numbers = [1, 4, 5, 2, 1];
    let num_slice = &mut numbers[1..=3];
    num_slice[1] = 555; 
    println!("As we can see, slice type can be mutable using indexing, {:?}", num_slice);

    let str4 = my_string3.as_str();
    println!("One more valid method to convert string to &str type {}", str4);
    let str5 = my_string2.as_mut_str();
    str5.make_ascii_uppercase();
    println!("{}",str5);
    let iter_string = String::from("ItErAtOr👋");
    let ch = iter_string.chars().nth(100);
    // let ch = iter_string.chars().nth(1);
    //returns option T so we need to do pattern matching
    if let Some(c) = ch{
        println!("{:?}", c);
    }else {
        println!("No such character");
    }
}

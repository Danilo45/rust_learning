fn main() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    println!("{}", s);  
}

fn make_uppercase(s: &mut String) {
    let upper = s.to_uppercase(); //to_uppercase creates and returns a new string, without modifying the original, and works for any UNICODE characters 
    *s = upper;
    /*
    If we work with only ascii letters , there is a simplier way using make_ascii_uppercase(). 
    That method modifies the string s without allocating a new string:
    
    s.make_ascii_uppercase();
    */
}
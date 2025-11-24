// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
    
//     println!("{}", s1);  // Will this compile?
//}    
//No, this will not compile, because s1 was moved into s2.
// After the move, s1 is no longer valid.
//To fix this, we can clone() s1 to create a new independent copy of the string
//We can also use a reference to borrow data without moving ownership.  

fn fix1(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("{}", s1); 
}
fn fix2(){
    let s1 = String::from("hello");
    let s2 = &s1;
    
    println!("{}", s1); 
}
fn fix3(){
    let s1 = String::from("hello");
    let s2 = s1;
    
    println!("{}", s2); 
}

fn main(){
    fix1();
    fix2();
    fix3();
}
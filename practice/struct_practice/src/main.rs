//implementing traits:
#[derive(Debug, Default)] 
// Debug trait is for printing full struct using {":?"} ...
// Default trint is for giving default values for Structs
struct TryDefault {
    member: String,
    number: u32,
    dec_num: f32
}

struct PascalCaseName{
    snake_case: String,
    number: u16,
}

struct Person {
    name: String,
    age: u32, 
    addr: String
}

fn main() {
    //creating instance of the  struct: 
    let test = PascalCaseName{
        snake_case: "Structs in Rust".to_string(),
        number: 24
    };
    println!("{:?}, {}", test.snake_case, test.number);

    let name = "Danilo".to_string(); 
    let age = 24;
    let addr = "Novi Sad, 21000".to_string();
    let person1 = Person {
        name,
        age,
        addr,
    };
    println!("My name is {:?}, and I am {} yo. I live in {:?}", person1.name, person1.age, person1.addr);

    //now if I have another person, with same age and sam address, than I can do this :
    let person2 = Person {
        name: "Marco".to_string(),
        ..person1
    };
    println!("His name is {:?}, and he is {} yo. He lives in {:?}", person2.name, person2.age, person2.addr);

    let try_default = TryDefault::default(); //this works because of Default trait
    println!("Default struct is : {:?}", try_default); //this works because of Debug trait
}

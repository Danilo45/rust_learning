fn main() {
    // Integers (examples with limit values)
    let int_8: i8 = -128;
    let uint_8: u8 = 255;
    let int_16: i16 = -32768;
    let uint_16: u16 = 65535;
    let int_32: i32 = -2_147_483_648;
    let uint_32: u32 = 4_294_967_295;
    let int_64: i64 = -9_223_372_036_854_775_808;
    let uint_64: u64 = 18_446_744_073_709_551_615;
    let int_size: isize = -123;
    let uint_size: usize = 123;
    println!("---------------------------------------");
    println!("Integers:");
    println!("i8: {}, u8: {}, i16: {}, u16: {}", int_8, uint_8, int_16, uint_16);
    println!("i32: {}, u32: {}\ni64: {}, u64: {}", int_32, uint_32, int_64, uint_64);
    println!("isize: {}, usize: {}", int_size, uint_size);

    // Floating-point numbers
    let pi: f32 = 3.14;
    let euler: f64 = 2.718281828459045;
    println!("---------------------------------------");
    println!("Floats:");
    println!("f32: {}, f64: {}", pi, euler);

    // Boolean
    let t = true;
    let f = false;
    println!("---------------------------------------");
    println!("Booleans:");
    if euler < 3.0{
        println!("Condition is {}, Euler's number is smaller than 3!", t);
    }
    if pi>100.0 {
        println!("Condition is {}, Pi is greater than 100!", t);
    }else {
        println!("Condition is {}, Pi is not greater than 100!", f);
    }
 
    println!("true AND false = {}", t && f);
    println!("true OR false = {}", t || f);
    println!("NOT true = {}", !t);

    // Characters
    let ch1 = 'D';
    let ch2 = 't';
    let ch3 = '🦀';
    let ch4 = '°';
    println!("---------------------------------------");
    println!("Characters:");
    println!("{} {} {} {}", ch1, ch2, ch3, ch4);
}

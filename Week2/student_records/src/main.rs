fn main() {
    let s1 = ("Danilo", 24, 9.35);
    let s2 = ("Marko", 22, 9.10);
    let s3 = ("Milica", 25, 7.80);

    let students = [s1, s2, s3];
    // {[argument][:[fill][align][width][.precision][type]]}
    println!("{:<12} | {:>3} | {:>7}", "Name", "Years", "Average");
    println!("{:-<12}-+-{:-<3}-+-{:-<7}", "", "", "");

    let mut sum: f32 = 0.0;
    for (name, age, avg) in students {
        println!("{:<12} | {:>3} | {:>7.2}", name, age, avg);
        sum += avg;
    }

    let class_avg = sum / students.len() as f32;

    println!("\nAverage: {:.2}", class_avg);
}

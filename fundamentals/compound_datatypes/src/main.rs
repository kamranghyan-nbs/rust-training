fn main() {
    let student: (u32, char, f64) = (25, 'A', 90.0);

    println!("{}", student.0);
    println!("{}", student.1);
    println!("{}", student.2);

    // Destructure
     println!("Destructure");
    let (age, grade, score) = student;
    println!("{}", age);
    println!("{}", grade);
    println!("{}", score);
}

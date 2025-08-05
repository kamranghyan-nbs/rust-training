fn main() {
    // let s = String::new();
    // let s1 = String::from("Welcome to NBS");

    // println!("{}", s);
    // println!("{}", s1);

    // Convert from str to string
    // let s = "Hello World";
    // let data = s.to_string();

    // println!("{}", data);

    // let data = "Pakistan".to_string();

    // println!("{}", data);

    // Push value into the string

    // let mut s = String::from("foo");
    // s.push_str(" bar");

    // println!("{}", s);

    // Push a character in string

    // let mut s = String::from("appl");
    // s.push('e');

    // println!("{}", s);

    // Concat by operator
    // let s1 = String::from("Tic");
    // let s2 = String::from("Toc");
    // let s3 = String::from("Tac");

    // let s = s1 + "-" + &s2 + "-" + &s3 ;
    // println!("{}", s)

    // Concat by format macro

    // let s1 = String::from("Tic");
    // let s2 = String::from("Toc");
    // let s3 = String::from("Tac");

    // let s = format!("{} - {} - {}", s1, s2, s3);
    // println!("{}", s);

    //  Str indexing
    let s = "Pakistan";
    let index_data = &s[0..4];
    println!("{}", index_data)


}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![2,8,10,20];

    v.push(10);
    v.push(20);
    v.push(30);
    v.pop();

    println!("Vector by Object : {:?}", v);
    println!("Vector by value : {:?}", v1);

    // access vector 
    // let element_1 = v1[3];
    // let element_1 = &v1[3];
    // let element_1 = v1.get(3);
    // println!("Vector by element : {:?}", element_1);

    // match element_1 {
    //     Some(value) => println!("{}", value),
    //     None => println!("nothing")
    // }

    // Loop on vectors
    // for i in v1 {
    //     println!("{}", i);
    // }
    // pass by reference
    // for i in &v1 {
    //     println!("{}", i);
    // }

    // mutable reference to change the values
    // for i in &mut v1 {
    //     *i += 10
    // }

    // println!("{:?}", v1);

    // Use enum to store multiple data types in vector
    let row = vec![SpreadSheetCell::Int(50), SpreadSheetCell::Float(10.6), SpreadSheetCell::Text(String::from("Hello world"))];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

// fn main() {
//     println!("Baker Sharing recipe with 1st person");
//     paper();
//     println!("#####################################");
//     println!("Baker Sharing recipe with 1st person");
//     paper();
//     println!("#####################################");
//     println!("Baker Sharing recipe with 1st person");
//     paper();
//     println!("#####################################");
// }

// fn paper(){
//     println!("1. Add Milk");
//     println!("2. Add Butter");
//     println!("3. Add Eggs");
//     println!("4. Add Sugar");
//     println!("5. Stir it");
//     println!("6. Heat on gentle flame");
// }

fn main() {
    let (value, value_1) = square(5,6);
    println!("{}", value);
    println!("{}", value_1);
}

// fn square(x:u32, y:u32){
//     let result = x * x;
//     let result_1 = y * y;

//     println!("Square of no {} is {}", x, result);
//     println!("Square of no {} is {}", y, result_1);
// }

fn square(x:u32, y:u32) -> (u32, u32) {
    let result = x * x;
    let result_1 = y * y;
    (result, result_1)
}



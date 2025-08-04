fn main() {

    let mut counter = 0;
    // loop {
    //     println!("Hello, world!");

    //     counter = counter + 1;

    //     if(counter == 3) {
    //         break
    //     }
    // }

    // while counter < 3 {
    //     println!("Hello, world!");
    //     counter = counter + 1;
    // }

    let lottery_number = [1, 23, 45, 6, 78, 9];

    // while 0 < lottery_number.len() {
    //     println!("{}", lottery_number[counter]);
    //     counter = counter + 1;
    // }

    // for a in 0..3 {
    //     println!("{}, Hello, world!", a);
    // }

    // for a in (0..3).rev() {
    //     println!("{}, Hello, world!", a);
    // }

    for element in lottery_number.iter() {
        println!("The value is {}", element);
    }
}

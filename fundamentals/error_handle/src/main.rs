use std::fs::File;
  //crate//module//struct

fn main() {

    // Panic error
    // let v = vec![1,2,3];

    // v[89];

    // Recorable error
    // let f = File::open("file.txt"); // result variant Ok variant Err

    // let f = match f {
    //     Ok(T) => T,
    //     Err(E) => {
    //         panic!("File not found {:#?}", E);
    //     },
    // };

    // Unwrap
    // let f = File::open("file.txt").unwrap();

    // expect
    let f = File::open("file.txt").expect("The file not found hello.txt");
}

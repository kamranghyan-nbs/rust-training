fn main() {

    // Deep Copy
    // let mut a = String::from("Hello! World");
    // a.push_str(" People");
    // println!("{}", a);

    // let s1 = String::from("Hello Duniya");
    // let s2 = s1.clone();

    // println!("s1: {}", s1);
    // println!("s2: {}", s2);

    // Function Ownership
    // let s = String::from("Hello");
    // take_ownership(s);

    // let x = 5;
    // make_copy(x);
    // println!("From the main fn: {}", x);

    // Returning Values & Scope

    // let result = gives_ownership();
    // println!("Result: {}", result);

    // let s1 = String::from("Pakistan");
    // let result1 = takes_and_gives_back(s1);

    // println!("Result 2: {}", result1)

    // Multiple value returns ownership

    let s = String::from("Pakistan");
    let (result, result_1) =  lenght(s);
    println!("The length of value {} is {}", result_1, result);

}

fn lenght(s: String) -> (usize, String) {
    (s.len(), s)
}

fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn take_ownership(x: String) {
    println!("{}", x);
}

fn make_copy(x: i32) {
    println!("{}", x);
}

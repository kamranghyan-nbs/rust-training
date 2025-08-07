#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn print_value<T: std::fmt::Debug>(value: MyOption<T>) {
    match value {
        MyOption::Some(v) => println!("Value is: {:?}", v),
        MyOption::None => println!("No value"),
    }
}

fn main() {
    let val1 = MyOption::Some(42);
    let val2: MyOption<i32> = MyOption::None;

    print_value(val1);
    print_value(val2);
}

#[derive(Debug)]

enum Student {
    Online,
    Onsite
}

#[derive(Debug)]
enum IPAdress {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move{x: i32, y:i32},
    ChangeColor(u32, u32, u32)
}

impl Message {
    fn call (&self) {
        println!("{:?}", self)
    }
}


#[derive(Debug)]

enum Option <T> {
    Some(T),
    None
}

fn main() {
    // let student_online = Student::Online;
    // let student_onsite = Student::Onsite;

    // println!("{:?}", student_online);
    // println!("{:?}", student_onsite);

    // Assign values to enum
    // let ip_v4 = IPAdress::V4(String::from("12.0.0.1"));
    // let ip_v6 = IPAdress::V6(String::from("::1"));

    // println!("{:?}", ip_v4);
    // println!("{:?}", ip_v6);

    // Variants of Enum

    // let msg_quit = Message::Quit;
    // let msg_write = Message::Write(String::from("Hello writing"));
    // let msg_move = Message::Move{x: 10, y: 20};
    // let msg_change_color = Message::ChangeColor(10, 20, 30);

    // println!("{:?}", msg_quit);
    // println!("{:?}", msg_write);
    // println!("{:?}", msg_move);
    // println!("{:?}", msg_change_color);

    // Define Method of Impl
    // let msg_write = Message::Write(String::from("Hello writing"));
    // let msg_move = Message::Move{x: 10, y: 20};

    // msg_write.call();
    // msg_move.call();

    // Generics Data Type Enum
    let some_number = Option::Some(5);
    let some_string = Option::Some(String::from("Hello writing"));

    println!("Generic Enum number: {:?}", some_number);
    println!("Generic Enum string: {:?}", some_string);



}

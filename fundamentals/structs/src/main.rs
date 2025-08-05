 #[derive(Debug)]

struct Book {
    name : String,
    author: String,
    price: u16,
    availability: bool,
}

struct Rectangle {
    height: u32,
    width: u32,
}

// impl Rectangle {
//     fn area (&self) -> u32 {
//         self.height*self.width
//     }
// }

impl Rectangle {
    fn area (&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {

    // let book_1 = Book {
    //     name: String::from("Name A"),
    //     author: String::from("Author A"),
    //     price: 10,
    //     availability: true,
    // };

    // let book_2 = Book {
    //     name: String::from("Name B"),
    //     author: String::from("Author B"),
    //    ..book_1
    // };

    // println!("{:#?}", book_1);
    // println!("{:#?}", book_2);


    // let height =  100;
    // let width =  50;

    // println!("The are of the square is: {}", area(height, width));  

    // Refactor with tuple

    // let rect1 = (100,50);
    // println!("The are of the square is: {}", diemensions(rect1));

    // Refactor by struct

    // let rect_1 = Rectangle {
    //     height: 100,
    //     width: 50,
    // };

    // println!("The are of the square is: {}", diemensions_struct(rect_1));

    // Method Implementation
    // let rec_1 = Rectangle { height: 100, width: 50};
    // let rec_2 = Rectangle { height: 50, width: 25 };

    // let result_1 = rec_1.area();
    // let result_2 = rec_2.area();

    // println!("The area of the square 1 is: {}", result_1);
    // println!("The area of the square 2 is: {}", result_2);

    // Method Implementation with arguments
    let rec_1 = Rectangle { height: 100, width: 50};
    let rec_2 = Rectangle { height: 90, width: 40 };
    let rec_3 = Rectangle { height: 80, width: 30 };

    let result_1 = rec_1.area(&rec_2);

    println!("Rec_1 can hold the Rec_2: {}", result_1);   
}

fn diemensions_struct(rect: Rectangle) -> u32 {
    rect.height * rect.width
}

fn diemensions(Diemensions: (u32, u32)) -> u32 {
    Diemensions.0 * Diemensions.1
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}

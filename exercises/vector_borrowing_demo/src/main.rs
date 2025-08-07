fn first_element<T>(vec: &Vec<T>) -> Option<&T> {
    vec.first()
}

fn main() {
    let numbers = vec![10, 20, 30];
    let string = vec!["King", "Queen", "Joker"];
    if let Some(first) = first_element(&string) {
        println!("The first element is: {}", first);
    } else {
        println!("The vector is empty.");
    }
}


use std::collections::HashMap;

fn main() {
    // let mut map = HashMap::new();

    // map.insert(String::from("Blue"), 10);
    // map.insert(String::from("Yellow"), 20);

    // println!("{:?}", map);

    // Hashmap using collect
    // let teams = vec![String::from("Blue"), String::from("Greem")];
    // let scores = vec![15,18];

    // let map: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();

    // println!("{:?}", map);

    // Ownership

    let field_key = String::from("color");
    let field_value = String::from("Blue");

    // let field_key = "color";
    // let field_value = "blue";

    let mut map = HashMap::new();

    map.insert(field_key, field_value);

    // println!("{:?}", map);

    // println!("{:?}", field_key);
    // println!("{:?}", field_value);

    // Access value of HashMap
    // let access_key = String::from("color");
    // let result = map.get(&access_key);

    // println!("{:?}", result);

    // Access HashMap via Loop
    for (key, value) in &map {
        println!("{} - {}", key, value );
    }

}

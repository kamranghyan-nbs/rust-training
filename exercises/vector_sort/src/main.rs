fn main() {
    // Step 1: Create a vector
    let mut numbers = vec![5, 3, 9, 1, 7];

    // Step 2: Sort the vector in ascending order
    numbers.sort();

    // Step 3: Print the sorted vector
    println!("Sorted vector: {:?}", numbers);

    // Step 4: Slice the vector from index 1 to 3 (inclusive)
    let sub_vector = &numbers[1..=3];

    // Step 5: Print the sub-vector
    println!("Sub-vector (index 1 to 3): {:?}", sub_vector);
}


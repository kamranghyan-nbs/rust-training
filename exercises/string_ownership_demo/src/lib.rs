pub fn print_owned_string(text: String) {
    println!("Printing owned string: {}", text);
    // The string is automatically dropped when this function ends
}

pub fn print_and_get_length(text: String) -> usize {
    println!("String content: '{}'", text);
    let length = text.len();
    println!("String length: {}", length);
    // Return length before string is dropped
    length
}

pub fn print_transformed_string(mut text: String) {
    // We can modify it since we own it
    text = text.to_uppercase();
    text.push_str("!!!");
    
    println!("Transformed string: {}", text);
}


pub fn print_multiple_owned_strings(strings: Vec<String>) {
    println!("Printing {} owned strings:", strings.len());
    for (index, string) in strings.into_iter().enumerate() {
        println!("  {}: {}", index + 1, string);
    }
    // All strings are dropped here
}

pub fn compare_owned_vs_borrowed(owned: String, borrowed: &str) {
    println!("Owned string: '{}' (length: {})", owned, owned.len());
    println!("Borrowed string: '{}' (length: {})", borrowed, borrowed.len());
    
    // We can modify the owned string
    // owned.push_str(" - modified!"); // Uncomment to see compilation error (owned is not mutable)
    
    // We cannot modify the borrowed string
    // borrowed.push_str(" - modified!"); // This would cause a compilation error
}


pub fn process_owned_string<F>(text: String, processor: F) 
where
    F: FnOnce(String),
{
    println!("About to process string: '{}'", text);
    processor(text);
    println!("String processing completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_owned_string() {
        let test_string = String::from("Test string");
        print_owned_string(test_string);
        // test_string is no longer accessible here
    }

    #[test]
    fn test_print_and_get_length() {
        let test_string = String::from("Hello");
        let length = print_and_get_length(test_string);
        assert_eq!(length, 5);
    }

    #[test]
    fn test_print_transformed_string() {
        let test_string = String::from("hello world");
        print_transformed_string(test_string);
        // Should print "HELLO WORLD!!!"
    }

    #[test]
    fn test_multiple_strings() {
        let strings = vec![
            String::from("First"),
            String::from("Second"),
            String::from("Third"),
        ];
        print_multiple_owned_strings(strings);
    }

    #[test]
    fn test_compare_owned_vs_borrowed() {
        let owned = String::from("I am owned");
        let borrowed = "I am borrowed";
        compare_owned_vs_borrowed(owned, borrowed);
    }

    #[test]
    fn test_process_owned_string() {
        let test_string = String::from("Process me");
        process_owned_string(test_string, |s| {
            assert_eq!(s, "Process me");
        });
    }
}
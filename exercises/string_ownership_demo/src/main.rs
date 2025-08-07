use string_ownership_demo::{
    print_owned_string,
    print_and_get_length,
    print_transformed_string,
    print_multiple_owned_strings,
    compare_owned_vs_borrowed,
    process_owned_string,
};

fn main() {
    println!("=== String Ownership Demonstration ===\n");

    // Example 1: Basic ownership transfer
    demonstrate_basic_ownership();

    // Example 2: Ownership with return values
    demonstrate_ownership_with_return();

    // Example 3: String transformation
    demonstrate_string_transformation();

    // Example 4: Multiple strings
    demonstrate_multiple_strings();

    // Example 5: Owned vs borrowed comparison
    demonstrate_owned_vs_borrowed();

    // Example 6: Ownership in closures
    // demonstrate_closure_ownership();

    // Example 7: What happens after ownership transfer
    // demonstrate_ownership_rules();
}

fn demonstrate_basic_ownership() {
    println!("1. Basic Ownership Transfer:");
    
    let my_string = String::from("Hello, Rust ownership!");
    println!("   Before: my_string = '{}'", my_string);
    
    // Transfer ownership to the function
    print_owned_string(my_string);
    
    // Uncommenting the next line would cause a compilation error:
    // println!("   After: my_string = '{}'", my_string); // ERROR: value used after move
    
    println!("   ✓ Ownership successfully transferred and string was printed");
    println!();
}

fn demonstrate_ownership_with_return() {
    println!("2. Ownership with Return Values:");
    
    let my_string = String::from("Count my characters");
    println!("   Original string: '{}'", my_string);
    
    // Transfer ownership but get a value back
    let length = print_and_get_length(my_string);
    println!("   ✓ Function returned length: {}", length);
    
    // my_string is still not accessible here
    println!();
}

fn demonstrate_string_transformation() {
    println!("3. String Transformation:");
    
    let my_string = String::from("transform me");
    println!("   Original: '{}'", my_string);
    
    // Function takes ownership and modifies the string
    print_transformed_string(my_string);
    
    println!("   ✓ String was transformed and printed");
    println!();
}

fn demonstrate_multiple_strings() {
    println!("4. Multiple String Ownership:");
    
    let strings = vec![
        String::from("First string"),
        String::from("Second string"), 
        String::from("Third string"),
    ];
    
    println!("   Created {} strings", strings.len());
    
    // Transfer ownership of the entire vector
    print_multiple_owned_strings(strings);
    
    // strings vector is no longer accessible here
    println!("   ✓ All strings were transferred and printed");
    println!();
}

fn demonstrate_owned_vs_borrowed() {
    println!("5. Owned vs Borrowed Strings:");
    
    let owned_string = String::from("I am heap-allocated");
    let borrowed_string = "I am string literal";
    
    println!("   Created owned and borrowed strings");
    
    // Function takes ownership of owned_string but only borrows borrowed_string
    compare_owned_vs_borrowed(owned_string, borrowed_string);
    
    // owned_string is no longer accessible
    // borrowed_string is still accessible
    println!("   Borrowed string still available: '{}'", borrowed_string);
    println!();
}

fn demonstrate_closure_ownership() {
    println!("6. Ownership in Closures:");
    
    let my_string = String::from("Process this string");
    println!("   Original: '{}'", my_string);
    
    // Pass ownership to a function that uses a closure
    process_owned_string(my_string, |owned_string| {
        let upper = owned_string.to_uppercase();
        let with_prefix = format!("PROCESSED: {}", upper);
        println!("   Closure result: '{}'", with_prefix);
    });
    
    println!("   ✓ String was processed by closure");
    println!();
}

fn demonstrate_ownership_rules() {
    println!("7. Ownership Rules Demonstration:");
    
    // Rule 1: Each value has exactly one owner
    let string1 = String::from("Original owner");
    println!("   string1 owns: '{}'", string1);
    
    // Rule 2: When owner goes out of scope, value is dropped
    {
        let string2 = String::from("Scoped string");
        println!("   string2 owns: '{}'", string2);
        print_owned_string(string2); // Ownership transferred
    } // string2 would be dropped here if it still existed
    
    // Rule 3: Ownership can be transferred
    let string3 = String::from("Will be moved");
    let string4 = string3; // Ownership moved from string3 to string4
    
    // This would cause error: println!("string3: {}", string3);
    println!("   string4 now owns: '{}'", string4);
    
    // Transfer to function
    print_owned_string(string4);
    // string4 is no longer accessible
    
    println!("   ✓ Demonstrated ownership transfer and scope rules");
    println!();
    
    // Creating new strings for final demonstration
    println!("8. Safe Patterns:");
    
    // Pattern 1: Clone if you need to keep the original
    let original = String::from("Keep me around");
    let copy_for_function = original.clone();
    print_owned_string(copy_for_function);
    println!("   Original still available: '{}'", original);
    
    // Pattern 2: Use references when you don't need to transfer ownership
    fn print_borrowed_string(text: &str) {
        println!("   Borrowed and printed: '{}'", text);
    }
    
    print_borrowed_string(&original);
    println!("   Original still available after borrowing: '{}'", original);
}
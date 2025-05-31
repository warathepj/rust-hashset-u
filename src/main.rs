use std::collections::HashSet; // We need to bring HashSet into scope

fn main() {
    // 1. Creating a new HashSet
    let mut my_set = HashSet::new(); // `mut` is important because we'll be adding to it

    println!("Initial set: {:?}", my_set); // Output: Initial set: {}

    // 2. Inserting elements
    my_set.insert("apple");
    my_set.insert("banana");
    my_set.insert("orange");
    my_set.insert("apple"); // Trying to insert "apple" again (it won't be duplicated)

    println!("Set after insertions: {:?}", my_set);
    // Output (order might vary, as HashSet doesn't guarantee order):
    // Set after insertions: {"orange", "apple", "banana"}
    // or {"banana", "orange", "apple"} etc.

    // 3. Checking for an element's existence
    if my_set.contains("banana") {
        println!("'banana' is in the set!");
    } else {
        println!("'banana' is NOT in the set.");
    }
    // Output: 'banana' is in the set!

    if my_set.contains("grape") {
        println!("'grape' is in the set!");
    } else {
        println!("'grape' is NOT in the set.");
    }
    // Output: 'grape' is NOT in the set.

    // 4. Removing an element
    my_set.remove("banana");

    println!("Set after removing 'banana': {:?}", my_set);
    // Output (order might vary):
    // Set after removing 'banana': {"orange", "apple"}

    // 5. Getting the number of elements
    println!("Number of elements in the set: {}", my_set.len());
    // Output: Number of elements in the set: 2

    // 6. Iterating over elements
    println!("Elements in the set:");
    for fruit in &my_set { // We iterate over references to avoid moving elements out
        println!("- {}", fruit);
    }
    // Output (order might vary):
    // Elements in the set:
    // - orange
    // - apple
}
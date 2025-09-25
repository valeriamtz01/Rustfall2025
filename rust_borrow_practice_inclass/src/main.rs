// Problem #1: String Concatenation with Borrowing
// Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();

    result.push_str(s1);
    result.push_str(s2);
    result
}

// Problem #2: Clone and Modify
// Given a string, clone it and modify the cloned string by appending a new word. 
// Print both the original string and the cloned, modified string to show that the original has not been changed.

fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}

// Problem #3: Mutable Reference Sum
// Write a modified sum function that takes a mutable reference for the destination of the sum from low to high.

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    let result: i32 = (low..=high).sum();
    *total = result;


}

fn main() {

    // PROBLEM #1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    // PROBLEM #2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    // PROBLEM #3
     // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total) //5050
}



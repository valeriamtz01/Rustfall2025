// Problem #1: String Concatenation with Borrowing
// Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new(); // creating a new string to store the "result"

    result.push_str(s1); // adds the first string to the result
    result.push_str(s2); // adds the second string to the result
    result // returns the concatenates string
}

// Problem #2: Clone and Modify
// Given a string, clone it and modify the cloned string by appending a new word. 
// Print both the original string and the cloned, modified string to show that the original has not been changed.

fn clone_and_modify(s: &String) -> String {
    let mut clone = s.clone(); // making a copy of the borrowed string
    clone.push_str("World!"); // modify word to the cloned string
    clone // returns the modified cloned string
}

// Problem #3: Mutable Reference Sum
// Write a modified sum function that takes a mutable reference for the destination of the sum from low to high.

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    let mut result = 0; // a temp value that stores the sum

    let mut i = low; // start from the lowest value

    while i <= high // going to loop from low to high while adding the number to the result
    {
        result = result + i; // adding the current number to the sum
        i = i+1; // moving to the next number
    }
    *total = result; // stores the sum in the varirable pointed by reference


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



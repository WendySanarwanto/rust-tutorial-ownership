use std::io;

// A helper to pause the program , waiting the user pressing enter, before moving to next demo
fn pause() {
    let stdin = io::stdin();
    println!("\nPress enter to continue to next demo.\n");
    let mut dummy = String::new();
    stdin.read_line(&mut dummy)
        .expect("Just press enter please.");
}

// Perform demo #1: Move ownership
fn move_demo() {
    println!("1. Ways Variables and Data Interact: Move");
    println!("-----------------------------------------\n");

    let s1 = String::from("hello");
    println!("Given, 's1' is '{}' and it is allocated in the Heap.", s1);
    let s2 = s1; //s1 is disabled, because the ownership has been moved to s2.
    println!("When, we have 's2' and we assign 's1' to 's2'.");
    // println!("{}, world!\n", s1); // If you uncomment this, compile error occurs.
    println!("Then, s2 = '{}, world!'", s2); // No error happens
    println!("If you try to print 's1', you should get compile error and this program will not run.\n");
    pause();
}

// Perform demo #2: Clone (deep copy) a variable that is allocated in the heap
fn clone_demo() {
    println!("2. Ways Variables and Data Interact: Clone");
    println!("-----------------------------------------\n");
    
    let s1 = String::from("hello world!");
    println!("Given, 's1' is '{}' and it is allocated in the Heap.", s1);
    println!("When, we deep copy 's1' into 's2' by using 'clone' method, ");
    let s2 = s1.clone();
    println!("Then, 's2' is '{}'. The content of 's1' is duplicated in the heap, the duplicated content's pointer is assign to 's2'.", s2);

    pause();
}

// A helper to support demo #3. It print the arg then return the arg back to the caller.
fn display_text(text: String) -> String {
    println!("----> 'display_text' displays the 'text' argument as '{}'\n", text);
    text
}

// Perform demo #3: Assign a heap variable into a function's argument will also move the variable's ownership.
fn function_demo() {
    println!("3. Assign heap variable as a function's argument");
    println!("-------------------------------------------------\n");

    let s1 = String::from("hello world!");
    println!("Given, 's1' is '{}' and it is allocated in the Heap.", s1);
    println!("And we have 'display_text' method which takes a String type argument");
    println!("When we assign 's1' as the argument of 'display_text' method & call the method,");
    println!("Then the ownership of 's1' is also moved to 'display_text' method.");
    let s2 = display_text(s1);
    println!("When called 'display_text', we kept the returned value into 's2'. This means, the ownership has been transfered back to 's2' from 'display_text', now.");
    println!("----> The 's2' is '{}'\n", s2);
    // Try enable this following line and you'll get compile error
    // s1.push_str("\n");
    pause();
}

// A helper to get character length of a string
fn get_length(text: &String) -> usize {
    return text.len();
}

// // If you enable this line , it won't compile. Immutable reference is not allowed to be modified.
// // A helper to support changing a borrowed heap variable
// fn change_borrowed_value(value: &String) {
//     value.push_str(" Today is a beautiful day.");
// }

// Perform demo #4: Reference concept in Rust
fn reference_demo() {
    println!("4. References");
    println!("--------------\n");

    let s1 = String::from("hello world!");
    println!("Given, 's1' is '{}' and it is allocated in the Heap.", s1);
    println!("And we have 'get_length' method which takes reference to a String typed argument and returns the length of referred String variable.");
    println!("When we assign 's1' as the argument of 'get_length' method & call the method,");
    println!("Then the ownership of 's1' is not moved, and we can re-use 's1' in next line.\n");

    let length = get_length(&s1);
    println!("----> '{}' has length: {:?}", s1, length);

    pause();

    // // Next we try to modify the s1 through calling change_borrowed_value and pass s1 into its argument.
    // change_borrowed_value(&s1);
    // println!("\n---->'s1' now is '{}'", s1);
}

// Main entry
fn main() {
    println!("\nRust Ownership's Demo.");
    println!("=====================\n");

    move_demo();

    clone_demo();

    function_demo();

    reference_demo();

    dangle_no_dangle_demo();
}

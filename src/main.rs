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

// Main entry
fn main() {
    println!("\nRust Ownership's Demo.");
    println!("=====================\n");

    move_demo();

    clone_demo();
}

fn print_string(x: String) {
    println!("{x}")
}

fn print_and_return_string(x: String) -> String {
    println!("{x}");
    x
}

fn print_integer(x: i32) {
    print!("{x}");
}

fn get_strlen(x: &String) -> usize {
    x.len()
}

fn append_hello(x: &mut String) {
    x.push_str(" hello");
}

fn main() {
    // Note: Cases 1-3 is only valid for memory allocated on the heap
    // Heap variables: Size is unknown on compile time, they're dynamically allocated & more expensive (Ex. String)
    // Stack variables: Size is known on compile time, faster to make (Ex. integers)

    // Case 1
    let str = String::from("Hello");
    let str2 = str; // str2 makes str invalid since bcs of an ownership transfer (move)
    println!("{str2}"); // move works like a shallow copy but the original is invalidated

    // Case 2
    let mut s = String::from("Testing");
    println!("{s}");
    s = String::from("test"); // this drops the memory allocated from "Testing"
    println!("{s}");

    // Case 3
    let src_string = String::from("Real String");
    let dst_string = src_string.clone(); // Cloning creates a deep copy of src_string
    println!("Source: {src_string}, Cloned: {dst_string}"); // Heap data gets copied too

    // Case 4: Variables allocated on the stack has the copy trait and will be deep copied by default
    let num1 = 5;
    let num2 = num1;
    println!("Num1: {num1}, Num2: {num2}"); // This is valid because i32 is created on the Stack

    // Case 5: Functions can also take ownership
    let main_string = String::from("Main String");
    print_string(main_string); // The function takes ownership here
                               // println!("{main_string}"); // This results in an error

    let main_int = 5;
    print_integer(main_int);
    println!("\n{main_int}"); // This is fine because it gets copied

    let mut test_string = String::from("Test String");
    println!("Before Function: {test_string}");
    test_string = print_and_return_string(test_string);
    // In this case the function returns the ownership back
    println!("After Function: {test_string}");

    // Instead of needing to return the ownership everytime, we can instead use references
    let len = get_strlen(&test_string);
    println!("Strlen of test string: {len}");

    // To change the value when referencing it, we can also make mutable references
    append_hello(&mut test_string);
    println!("{test_string}");

    // Note: We can't have any other reference with a mutable reference
    // This prevents race conditions from happening

    // Extra Notes: Reference scopes
    let mut ref_string = String::from("Reference String");

    let r1 = &ref_string;
    let r2 = &ref_string;

    // let r3 = &mut ref_string;
    // This causes an error because r1 and r2 are still in scope
    // r1 and r2 are still in scope and the borrow checker still considers them as active borrows

    println!("Using R1 and R2: {r1}, {r2}");
    // This causes r1 and r2 to be used
    // The borrow checker now considers them to be inactive borrows which will ends their lifetime
    // But we can also use r1 & r2 multiple times, the borrow checker will extend their lifetime

    let r3 = &mut ref_string; // Now we can access $ref_string
    println!("Using R3: {r3}");
}

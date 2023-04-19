use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    // If entered index is out of array bounds, the program will panic to prevent undefined behavior.
    // Because, the compiler can't know if the index is valid or not in compile time.
    println!("The value of the element at index {index} is: {element}");
}
// A fundamental goal of Rust is to ensure that programs never have undefined behavior.

// The beneath code operates as each step:
// 1. At "word", the string "Hello" has been allocated on the heap. It is owned by "word".
// 2. At "add_suffix", the function has been called. This moves ownership of the string from "word" to "sentence" of the function. 
// The string data is not copied, but the pointer to the data is copied.
// 3. At the function "sentence.push_str(" World.")" resizes the string's heap allocation. This does three things. First, it creates a new larget allocation.
// Second, it writes "Hello World." into the new allocation. Third, it frees the original heap memory. "word" now points to deallocated memory.
// 4. The frame for "add_suffix" is gone. This function returned "sentence", transferring ownership of the string to "sentence" of main.

fn main() {
    let word = String::from("Hello");
    let sentence = add_suffix(word);
    println!("{sentence}");
}

fn add_suffix(mut sentence: String) -> String {
    sentence.push_str(" World.");
    sentence
}

// borrow checker:
//                  Read (R): data can be copied to another location
//                  Write (W): data can be mutated in-place
//                  Own (O): data can be moved or dropped
// These permissions don't exist at runtime, only within the compiler. They describe how the compiler "thinks"
// about my program before the program is executed.
// By default, a variable has read/own permissions (RO) on its data. If a variable is annotated with "let mut",
// then it also has the write permission (W). ***The key idea is that references can temporarily remove these permissions***.

fn first_or(strings: &Vec<String>, default: &String) -> &String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}

fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    // drop(default);
    println!("{}", s);
}
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // This following loop is not safe if you update an array with 6 elements, but the loop index is not updated. 
    // This will cause a runtime error. Also, this method is slow than the for loop. In the runtime, the compiler checks the index whether
    // it is far from the array bounds or not. So, it is not recommended to use this method.
    while index < 5 {
        println!("The value of the element at index {} is: {}", index, a[index]);
        index += 1;
    }

    // On the other side, Rustaceans would use a 'for' loop. The way uses the 'Range' type to create an iterator.
    for element in a {
        println!("The value of the element is: {}", element);
    }
}
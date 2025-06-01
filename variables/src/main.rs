use std::io;

fn main() {

    let arr:[i32;5] = [23,2,1,2,3];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Please enter an valid number!");

    let index : usize = index.trim().parse().expect("Please enter a valid number!");

    let ele = arr[index];
    println!("The value of the element at index {index} is: {ele}")
}
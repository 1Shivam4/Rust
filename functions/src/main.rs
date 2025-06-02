fn main() {
    println!("Hello, world!");

    another_function("hii".to_string());
    expression_function();

    let x = five();
    println!("Value of x is: {}", x);

    let y = plus_one(5);
    println!("Value of y is: {}", y);

}

fn another_function(str: String){
    println!("Hellow World !!!");
    println!("Received: {}", str)
}

// Expressions in a function

fn expression_function(){
    let y = {
        let x = 6;
        x + 6
    };
    println!("The value of y: {y}");
}

fn five() -> i32{
    5
}

// Return type function with parameter
fn plus_one(i: i32) -> i32{
    i + 1
}
fn main() {
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

//     let condition = true;

//     let x = if condition {5} else {6};

//     println!("Value of x : {}", x);
// }

    // let condition = true;

    // let number = if condition { 5 } else { "six" }; // this will cause an error
    // // arm1 = i32 and arm2 = string

    // println!("The value of number is: {number}");



    // Loops 
    // let mut counter = 0;
    
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("Value of result: {}", result);

    // Loop Labels to Disambiguate loops
//     let mut count = 0;
   
//    'counting_loop: loop{
//     println!("Count = {count}");
//     let mut remaining = 10;

//     loop{
//          println!("remaining = {remaining}");
//         if remaining == 9 {
//             break;
//         }

//         if count == 2{
//             break 'counting_loop;
//         }

//         remaining -= 1;
//     }

//     count += 1;
//    }
//     println!("End count = {count}");

// Using while loop

    // let mut number = 2;

    // while number != 0 {
    //     println!("number = {number}");

    //     number -= 1;
    // }

    

    // Using For loop to print or access Collection with for

    let a = [12,3,4,4,3,2,11];

    for ele in a{
        println!("Element = {ele}")
    }


    // Looping over a rnage of numbers 
    for i in (1..10).rev(){
         println!("i = {i}");
    }

    println!("LIFTOFF!!!");
}
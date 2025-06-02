use std::io;

// Fahrenheit and Celsius
// fn main(){
//     println!("Fahrenheit to Celsius");

//     let mut fareh = String::new();

//     io::stdin().read_line(&mut fareh).expect("Enter a valid temperature");

//     let fareh: f64 = fareh.trim().parse().expect("Temperature should be in numbers");

//     let celsius: f64 = (fareh - 32.0) * (5.0 / 9.0);

//     println!("Fahrenheit =  {} converted to celsius  = {:.2}",fareh, celsius);
// } 

// fn main(){
//     println!("Celsius to Fahrenheit");

//     let mut celsius = String::new();

//     io::stdin().read_line(&mut celsius).expect("Enter a valid temperature");

//     let celsius: f64 = celsius.trim().parse().expect("Temperature should be in numbers");

//     let fareh: f64 = (celsius * (9.0 / 5.0)) + 35.0;

//     println!("Degree celsius =  {} converted to  Fahrenheit  = {:.2}",celsius, fareh);
// } 


// fn main(){
//     println!("Enter the position of Nth Fibonacci series");

//     let mut pos = String::new();

//     io::stdin().read_line(&mut pos).expect("enter the valid pos!");

//     let pos: u32 = pos.trim().parse().expect("Position should be an positive number");

//     if pos == 0{
//         println!("Fibonacci number at position 0 is 0");
//         return;
//     }else if pos == 1{
//         println!("Fibonacci number at position 1 is 1");
//         return;
//     }

//     let (mut prev,mut next,mut result) = (0, 1, 1);

//     for _ in 2..=pos{ // this should be a valid u32 or for does not loop over u32 as u32 is not iterable
//         result = prev + next;
//         prev = next;
//         next = result;
//     };

//     println!("Value of Fibonacci number = {} at position = {:.2}", result, pos);
// }

fn main(){
    const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
];

const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree.",
    "two Turtle Doves,",
    "three French Hens,",
    "four Calling Birds,",
    "five Gold Rings,",
    "six Geese a Laying,",
    "seven Swans a Swimming,",
    "eight Maids a Milking,",
    "nine Ladies Dancing,",
    "ten Lords a Leaping,",
    "eleven Pipers Piping,",
    "twelve Drummers Drumming,"
];

for day in 0..12{
    println!("On {} day of Christmas", DAYS[day]);

    for gift in (0..=day).rev(){
        if gift == 0 && day != 0{
            println!("{}", GIFTS[gift]);
        }else{
            println!("{}", GIFTS[gift]);
        }
    }

    println!();
}

}
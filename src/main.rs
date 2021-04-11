//To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope. 
//std stands for standard librarys
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    println!("Pleae input your guess");

    //let statement, which is used to create a variable. 
    //mut means "mutable"
    //The :: syntax in the ::new line indicates that new is an associated function of the String type. 
    let mut guess = String::new();

    //call the stdin function from the io module:
    io::stdin()
        //The & indicates that this argument is a reference, which gives you a way to 
        //let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You won"),
    }
}

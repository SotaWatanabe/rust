//To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope. 
//std stands for standard librarys
use std::io;

fn main(){
    
    println!("Guess the number!");

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
    
    println!("You guessed: {}", guess);
}

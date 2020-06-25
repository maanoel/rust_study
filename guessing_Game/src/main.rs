use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){

	loop{
        println!("guess the number");
		
        let _screte_number = rand::thread_rng().gen_range(1,101);
			 
		let mut guess = String::new();
		
		io::stdin()
		.read_line(&mut guess) //& (it is to indicate that the mutable variable is to be passed through parameter)
		.expect("Failed to red line");
		
		let guess: u32 = guess.trim().parse().expect("please type a number");
		
		println!(" your guessed: {}", guess);
		
		match guess.cmp(&_screte_number){
			Ordering::Less => println!("Too less"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
			    println!("you win!")
				break;
			}
		}
		
		println!("the secret is {}", _screte_number);
	
	}
	
}
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){

	let _f: bool = false;
	
	my_tuple();
	my_array();

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
			    println!("you win!");
				break;
			}
		}
		
		if guess > 100 {
			println!("Its to high");
		}
		
		println!("the secret is {}", _screte_number);
		
	
	}
	
	fn my_tuple(){
		
		let  _tup = (1, 2, 3);
		let (_x, _y, _z ) = _tup;
		
		println!("the full tuple is: {} {} {}", _tup.0, _tup.1, _tup.2);
		println!("the vale is {}", _x);
	}
	
	fn my_array(){
		let _months = ["January", "February"];
		
		println!("the array index {}", _months[0]);
		
		crazy_function();
	}
	
	fn crazy_function() -> i32{
	    5
	}
	
	fn ternary(){
	    let condition = true;
		let value =  if condition {5} else {4};
	}
	
	fn teste_string(){
		let _value = String::from("hi");
	}
}
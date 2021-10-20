#[allow(unused_imports)]
use std::io;
use rand::Rng;
use rand::thread_rng;

fn main() {

    let mut rng = thread_rng();
    let secret = rng.gen_range(0, 10);
    println!("{}", secret);
    let mut guess = String::new();
    println!("Please enter a number");
    io::stdin().read_line(&mut guess).unwrap();
    let guess: u32 = guess.trim().parse().expect("not a valid number");
    println!("You guessed {}", guess);
/*    if guess>secret {
	println!("Too high");
    }else if guess<secret {
	println!("Too low");
    }else {
	println!("You've guessed the correct number");
    }
*/
}

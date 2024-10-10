use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
 
    let upper_limit = 25;
    let lower_limit = 1;

    let left_hand = rng.gen_range(lower_limit..=upper_limit);
    let right_hand = rng.gen_range(lower_limit..=upper_limit);
    let answer = left_hand + right_hand;

    print!("{} + {} = ", left_hand, right_hand);
    io::stdout().flush();

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line.");
    let guess: u32 = guess.trim().parse().expect("expected a number");

    match guess.cmp(&answer) {
        Ordering::Equal => print!("got it"),
        _ => println!("didn't got it")
    }
}

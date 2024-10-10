use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // set stats 
    let mut correct = 0;
    let mut mistake = 0;

    let mut rng = rand::thread_rng();
 
    // assign limits
    let upper_limit = 25;
    let lower_limit = 1;

    loop {
        // new attempt
        // assign left hand, right hand, and answer
        let left_hand = rng.gen_range(lower_limit..=upper_limit);
        let right_hand = rng.gen_range(lower_limit..=upper_limit);
        let answer = left_hand + right_hand;

        print!("{} + {} = ", left_hand, right_hand);
        io::stdout().flush(); // print equation

        // get input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        }

        match guess.cmp(&answer) {
            Ordering::Equal => correct += 1,
            _ => mistake += 1
        }
    }
    println!("{} correct, {} mistakes out of {}", correct, mistake, correct + mistake);
}

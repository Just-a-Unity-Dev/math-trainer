use std::time::{Duration,SystemTime};
use std::cmp::Ordering;
use std::io::Write;
use rand::Rng;
use std::io;

mod operator;

fn main() {
    // set stats 
    let mut correct = 0;
    let mut mistake = 0;

    let mut rng = rand::thread_rng();
 
    // assign limits
    let upper_limit = 10;
    let lower_limit = 1;

    let mut v: Vec<Duration> = Vec::new();

    loop {
        // new attempt
        // assign time started
        let attempt_started = SystemTime::now();

        // assign left hand, right hand, and answer
        let left_hand = rng.gen_range(lower_limit..=upper_limit);
        let right_hand = rng.gen_range(lower_limit..=upper_limit);
        let operator: operator::Operator = rand::random();

        let answer = match operator::operate(left_hand, right_hand, &operator) {
            Ok(n) => n,
            Err(_) => continue
        };

        print!("{} {} {} = ", left_hand, &operator.to_string(), right_hand);
        match io::stdout().flush() {
            Err(_) => break,
            Ok(_) => true
        }; // print equation

        // get input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        // if the line is not a number, assume the player wnats to stop playing
        // break at this specific point because we havent recorded any data yet 
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        };

        match guess.cmp(&answer) {
            Ordering::Equal => correct += 1,
            _ => mistake += 1
        }
        match SystemTime::now().duration_since(attempt_started) {
            Ok(duration) => v.push(duration),
            Err(_) => break
        }
    }

    let mut average_duration: u32 = 0;
    for attempt_duration in v.iter() {
        average_duration += attempt_duration.as_millis() as u32;
    }
    average_duration = average_duration / v.len() as u32;

    let total = correct + mistake;
    println!("score: {}/{}", correct, total);
    println!("average attempt lasted {} ms", average_duration);
}

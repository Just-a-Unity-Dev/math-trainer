use std::time::{Duration,Instant};
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
    let session_start = Instant::now();

    loop {
        // new attempt
        // assign time started
        let attempt_start = Instant::now();

        // assign left hand, right hand, and answer
        let left_hand = rng.gen_range(lower_limit..=upper_limit);
        let right_hand = rng.gen_range(lower_limit..=upper_limit);
        let operator: operator::Operator = rand::random();

        let Ok(answer) = operator::operate(left_hand, right_hand, &operator) else {
            continue;
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
        // record attempt start
        v.push(attempt_start.elapsed());
    }
    let total_answers = v.len();
    let average_duration: u128 = session_start.elapsed().as_millis() / v.len() as u128;
    let average_duration_correct: u128 = session_start.elapsed().as_millis() / correct as u128;

    let inner_message = match mistake {
        0 => "perfect! :D".to_string(),
        total_answers => format!("{} mistake(s) :( ", mistake),
        _ => format!("{} mistakes", mistake)
    };
    println!("score: {}/{}: {}", correct, v.len(), inner_message);
    println!("avg {}ms/correct attempt ({}ms/atmp)", average_duration_correct, average_duration);
    println!("total {}s", session_start.elapsed().as_secs());
}

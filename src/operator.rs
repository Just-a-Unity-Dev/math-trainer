use rand::{
    distributions::{Distribution, Standard},
    Rng
};
use std::fmt;

pub enum Operator {
    ADD,
    SUB,
    MUL
}

pub fn operate(left_hand: i32, right_hand: i32, operator: &Operator) -> Result<i32, &'static str> {
    match operator {
        Operator::ADD => Ok(left_hand + right_hand),
        Operator::SUB => Ok(left_hand - right_hand),
        Operator::MUL => Ok(left_hand * right_hand),
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operator::*;
        write!(f, "{}", 
            match self {
                ADD => "+",
                SUB => "-",
                MUL => "*",
            }.to_string()
        )
    }
}

impl Distribution<Operator> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operator {
        use Operator::*;
        match rng.gen_range(0..=2) {
            0 => ADD,
            1 => SUB,
            2 => MUL,
            _ => ADD
        }
    }
}

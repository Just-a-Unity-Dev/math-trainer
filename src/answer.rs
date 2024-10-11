use std::time::{Duration};
use crate::operator::{Operator, operate};

pub struct Answer {
    pub left_hand: i32,
    pub right_hand: i32,
    pub operator: Operator,
    pub actual_answer: i32,
    pub player_answer: i32,
    pub duration: Duration
}

pub fn get_result_from_answer(answer: &Answer) {
    operate(answer.left_hand, answer.right_hand, &answer.operator);
}

use std::io;
use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum Hand {
    Rock,
    Scissor,
    Paper,
}

#[derive(PartialEq)]
pub enum JankenResult {
    Win,
    Lose,
    Draw,
}

impl Hand {
    pub fn fight(me: &Hand, other: &Hand) -> JankenResult {
        match (me.clone(), other.clone()) {
            (Hand::Rock, Hand::Rock) => JankenResult::Draw,
            (Hand::Rock, Hand::Scissor) => JankenResult::Win,
            (Hand::Rock, Hand::Paper) => JankenResult::Lose,

            (Hand::Scissor, Hand::Rock) => JankenResult::Lose,
            (Hand::Scissor, Hand::Scissor) => JankenResult::Draw,
            (Hand::Scissor, Hand::Paper) => JankenResult::Win,

            (Hand::Paper, Hand::Rock) => JankenResult::Win,
            (Hand::Paper, Hand::Scissor) => JankenResult::Lose,
            (Hand::Paper, Hand::Paper) => JankenResult::Draw,
        }
    }

    pub fn clone(&self) -> Hand {
        match *self {
            Hand::Rock => Hand::Rock,
            Hand::Scissor => Hand::Scissor,
            Hand::Paper => Hand::Paper,
        }
    }
}


fn num_to_hand(num: i8) -> Result<Hand, String> {
    match num {
        0 => Ok(Hand::Rock),
        1 => Ok(Hand::Scissor),
        2 => Ok(Hand::Paper),
        _ => Err("ERRRR".to_owned()),
    }
}

pub fn get_hand(input: &mut String) -> Result<Hand, String> {
    io::stdin()
        .read_line(input)
        .expect("failed to read line.");
    *input = input.trim_end().to_owned(); 
    let input_num: i8 = input.parse().expect("input a number!");
    num_to_hand(input_num)
}


pub fn print_result(res: JankenResult) {
    if res == JankenResult::Win {
        println!("Hand1 won!");
    }

    else if res == JankenResult::Draw {
        println!("Draw!");
    }

    else {
        println!("Hand2 won!");
    }
}

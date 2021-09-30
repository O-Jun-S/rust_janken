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
    pub fn fight(&self, other: &Hand) -> JankenResult {
        match *self {
            Hand::Rock => self.rock_fight(other),
            Hand::Scissor => self.scissor_fight(other),
            Hand::Paper => self.paper_fight(other),
        }
    }

    fn rock_fight(&self, other: &Hand) -> JankenResult {
        match *other {
            Hand::Rock => JankenResult::Draw,
            Hand::Scissor => JankenResult::Win,
            Hand::Paper => JankenResult::Lose,
        }
    }

    fn scissor_fight(&self, other: &Hand) -> JankenResult {
        match *other {
            Hand::Rock => JankenResult::Lose,
            Hand::Scissor => JankenResult::Draw,
            Hand::Paper => JankenResult::Win,
        }
    }

    fn paper_fight(&self, other: &Hand) -> JankenResult {
        match *other {
            Hand::Rock => JankenResult::Win,
            Hand::Scissor => JankenResult::Lose,
            Hand::Paper => JankenResult::Draw,
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

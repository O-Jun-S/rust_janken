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

pub fn fight(hand: Hand, other: Hand) -> JankenResult {
    if hand == Hand::Rock {
        if other == Hand::Rock {
            JankenResult::Draw
        }

        else if other == Hand::Scissor {
            JankenResult::Win
        }

        else {
            JankenResult::Lose
        }

        
    }

    else if hand == Hand::Scissor {
        if other == Hand::Rock {
            JankenResult::Lose
        }

        else if other == Hand::Scissor {
            JankenResult::Draw
        }

        else {
            JankenResult::Win
        }
    }

    else {
        if other == Hand::Rock {
            JankenResult::Win
        }

        else if other == Hand::Scissor {
            JankenResult::Lose
        }

        else {
            JankenResult::Draw
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

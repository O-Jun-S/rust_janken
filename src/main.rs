use std::io;
use std::cmp::PartialEq;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Scissor,
    Paper,
}


#[derive(PartialEq)]
enum JankenResult {
    Win,
    Lose,
    Draw,
}


fn fight(hand: Hand, other: Hand) -> JankenResult {
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


fn get_hand(input: &mut String) -> Result<Hand, String> {
    io::stdin()
        .read_line(input)
        .expect("Failed to read line.");
    *input = input.trim_end().to_owned(); 
    let input_num: i8 = input.parse().expect("Input a number!");
    num_to_hand(input_num)
}


fn main() {
    let mut hand1 = String::new();
    let mut hand2 = String::new();

    let hand1_hand = get_hand(&mut hand1);
    let hand2_hand = get_hand(&mut hand2);

    let res = fight(hand1_hand.unwrap(), hand2_hand.unwrap());

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

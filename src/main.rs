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


fn main() {
    let mut hand1 = String::new();
    let mut hand2 = String::new();

    io::stdin()
        .read_line(&mut hand1)
        .expect("Failed to read line.");
    
    hand1 = hand1.trim_end().to_owned(); 
    let hand1_num: i8 = hand1.parse().expect("Input a number!");
    let hand1_hand = num_to_hand(hand1_num);

    io::stdin()
        .read_line(&mut hand2)
        .expect("Failed to read line.");
    hand2 = hand2.trim_end().to_owned();
    let hand2_num: i8 = hand2.parse().expect("Input a number!");
    let hand2_hand = num_to_hand(hand2_num);

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

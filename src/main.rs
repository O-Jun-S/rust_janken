use std::io::{stdout, Write};
mod janken;

fn description() {
    println!("Janken game.");
    println!("0: Rock");
    println!("1: Scissor");
    println!("2: Paper");
}


fn input_line(prompt: String) {
    print!("{}", prompt);
    stdout().flush().unwrap();
}


fn main() {
    description();

    input_line("Input first hand >> ".to_owned());
    let mut hand1 = String::new();
    let hand1_hand = janken::get_hand(&mut hand1).unwrap();

    input_line("Input second hand >> ".to_owned());
    let mut hand2 = String::new();
    let hand2_hand = janken::get_hand(&mut hand2).unwrap();

    let res = janken::Hand::fight(&hand1_hand, &hand2_hand);
    janken::print_result(res);
}

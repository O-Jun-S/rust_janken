mod hand;

fn main() {
    let mut hand1 = String::new();
    let mut hand2 = String::new();

    let hand1_hand = hand::get_hand(&mut hand1);
    let hand2_hand = hand::get_hand(&mut hand2);

    let res = hand::fight(hand1_hand.unwrap(), hand2_hand.unwrap());
    hand::print_result(res);
}

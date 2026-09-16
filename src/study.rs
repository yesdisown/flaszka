use std::io;

use crate::models::*;

/// starts a cli study session, with (count) amount of cards.
pub fn study(chest: &mut Chest, count: usize) {
    let mut count = count;
    let mut pending_moves = Vec::new();
    let chest_card_count = chest.card_count();
    if count > chest_card_count {
        count = chest_card_count
    }

    for i in 0..count {
        let card = match chest.get_top_card() {
            Some(c) => c,
            None => break,
        };

        let card_front = &card.front;
        let card_back = &card.back;

        println!("{i}/{count}\n{card_front}");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("\nCorrect answer: {card_back}\ndo you consider your answer correct? [y/n]");

        let is_correct = loop {
            let mut input_answer_confirmation = String::new();
            io::stdin()
                .read_line(&mut input_answer_confirmation)
                .unwrap();

            match input_answer_confirmation.trim() {
                "y" => break true,
                "n" => break false,
                _ => (),
            }
        };

        let target = match is_correct {
            true => MoveTarget::Next,
            false => MoveTarget::First,
        };

        let pending_move = chest.get_top_card_move(target).unwrap();
        pending_moves.push(pending_move);
    }

    for m in pending_moves {
        chest.move_card(m);
    }
}

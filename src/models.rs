use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct Card {
    pub front: String,
    pub back: String,
    pub fields: HashMap<String, String>,
}

pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
}

pub struct Shelf {
    pub id: u8,
    pub cards: Vec<Card>,
}

pub struct Chest {
    pub name: String,
    pub shelves: Vec<Shelf>,
}

impl Shelf {
    pub fn find_id_by_card(&self, card: Card) -> Option<usize> {
        self.cards.iter().position(|c| c == &card)
    }
}

impl Chest {
    pub fn get_flashcard(&mut self) -> Option<Card> {
        self.shelves.first_mut()?.cards.pop()
    }
}

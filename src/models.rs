use std::collections::HashMap;

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

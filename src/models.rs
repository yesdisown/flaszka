use std::collections::HashMap;

pub enum MoveTarget {
    First,
    Next,
}
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
    pub cards: Vec<Card>,
}

pub struct Chest {
    pub name: String,
    pub shelves: Vec<Shelf>,
}

impl Shelf {
    /// returns a refference to the first card of the shelf
    pub fn top_card(&self) -> Option<&Card> {
        self.cards.first()
    }
}

impl Chest {
    /// returns the id of the first non-empty shelf
    fn top_shelf_id(&self) -> Option<usize> {
        Some(
            self.shelves
                .iter()
                .enumerate()
                .find(|(_, s)| !s.cards.is_empty())?
                .0,
        )
    }

    /// moves the top card of the first non-empty shelf of the chest.
    /// returns None if chest is empty
    pub fn move_top_card(&mut self, target: MoveTarget) -> Option<()> {
        // because of the ? operator, if the chest is empty None will be returned here
        let top_shelf_id = self.top_shelf_id()?;
        let top_card = self.shelves.get_mut(top_shelf_id).unwrap().cards.remove(0);

        let target_shelf_id = match target {
            MoveTarget::First => 0,
            MoveTarget::Next if self.shelves.len() == top_shelf_id => top_shelf_id,
            MoveTarget::Next => top_shelf_id + 1,
        };

        self.shelves
            .get_mut(target_shelf_id)
            .unwrap()
            .cards
            .push(top_card);

        Some(())
    }
}

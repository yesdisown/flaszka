use std::collections::HashMap;

pub enum MoveTarget {
    First,
    Next,
}

pub struct PendingCardMove {
    pub shelf_id: usize,
    pub card: Card,
    pub target: MoveTarget,
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

    /// Takes a PendingCardMove and moves a card accordingly.
    /// returns None if the shelf_id corresponding shelf doesn't exist.
    pub fn move_card(&mut self, m: PendingCardMove) -> Option<()> {
        let target_shelf_id = match m.target {
            MoveTarget::First => 0,
            MoveTarget::Next if self.shelves.len() - 1 == m.shelf_id => m.shelf_id,
            MoveTarget::Next => m.shelf_id + 1,
        };

        self.shelves.get_mut(target_shelf_id)?.cards.push(m.card);

        Some(())
    }

    /// takes a MoveTarget, and returns a PendingCardMove for the top card.
    /// returns None, if the chest is empty.
    pub fn get_top_card_move(&mut self, target: MoveTarget) -> Option<PendingCardMove> {
        let top_shelf_id = self.top_shelf_id()?;
        let top_card = self.shelves.get_mut(top_shelf_id)?.cards.remove(0);

        Some(PendingCardMove {
            shelf_id: top_shelf_id,
            card: top_card,
            target,
        })
    }

    /// returns a refference to the first card of the first shelf
    pub fn get_top_card(&self) -> Option<&Card> {
        let top_shelf_id = self.top_shelf_id()?;
        let top_card = self.shelves.get(top_shelf_id)?.cards.first()?;

        Some(top_card)
    }

    pub fn card_count(&self) -> usize {
        let mut count: usize = 0;
        for shelf in &self.shelves {
            count += shelf.cards.len();
        }
        count
    }
}

use std::collections::HashMap;
use crate::weapon_item::WeaponItem;

/// Represents a player's inventory with a fixed number of slots.
///
/// # Fields
/// * `slots`: A HashMap representing inventory slots and their contents.
///
/// # Example
/// ```
/// let mut inventory = PlayerInventory::new(20);
/// let sword = WeaponItem {
///     id: 1,
///     name: "Iron Sword".to_string(),
///     description: "A basic iron sword".to_string(),
///     weight: 5.0,
///     value: 100,
/// };
/// inventory.add_item(0, sword);
/// ```
#[derive(Debug, Clone)]
pub struct PlayerInventory {
    pub slots: HashMap<u32, Option<WeaponItem>>,
}

impl PlayerInventory {
    pub fn new(num_slots: u32) -> Self {
        let mut slots = HashMap::new();
        for i in 0..num_slots {
            slots.insert(i, None);
        }
        Self { slots }
    }

    pub fn get_item(&self, slot: u32) -> Option<&WeaponItem> {
        self.slots.get(&slot).and_then(|item| item.as_ref())
    }

    pub fn add_item(&mut self, slot: u32, item: WeaponItem) {
        self.slots.insert(slot, Some(item));
    }

    pub fn remove_item(&mut self, slot: u32) -> Option<WeaponItem> {
        self.slots.insert(slot, None).flatten()
    }

    pub fn empty_slot(&mut self, slot: u32) {
        self.slots.insert(slot, None);
    }
}
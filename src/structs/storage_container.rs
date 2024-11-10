use serde::{Deserialize, Serialize};
use crate::player_inventory::PlayerInventory;
use uuid::Uuid;
/// Represents a storage container in the game world for weapons.
///
/// # Fields
/// * `uuid`: A unique identifier for the container.
/// * `inventory`: The inventory associated with this container.
///
/// # Example

/// let weapon_chest = StorageContainer::new(30);
/// let weapon = WeaponItem {
///     id: 1,
///     name: "Sword of Destiny".to_string(),
///     description: "A powerful legendary sword".to_string(),
///     weight: 10.0,
///     value: 1500,
/// };
/// weapon_chest.inventory.add_item(0, weapon);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageContainer {
    pub uuid: Uuid,
    pub inventory: PlayerInventory,
}

impl StorageContainer {
    pub fn new(num_slots: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inventory: PlayerInventory::new(num_slots),
        }
    }
}
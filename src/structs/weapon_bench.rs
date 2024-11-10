use super::{WeaponsList, WeaponEntry, WeaponPart, CraftPlugin};
use std::collections::HashMap;

pub struct WeaponCraftingBench {
    // Stores available weapon blueprints
    pub blueprints: WeaponsList,
    // Stores parts required for crafting
    pub parts: Vec<WeaponPart>,
}

impl WeaponCraftingBench {
    // Initialize the WeaponCraftingBench with default values
    pub fn new() -> Self {
        WeaponCraftingBench {
            blueprints: WeaponsList::new(),
            parts: vec![],
        }
    }

    // Function to add a weapon blueprint to the crafting bench
    pub fn add_blueprint(&mut self, blueprint: WeaponEntry) {
        self.blueprints.weapons.insert(blueprint.weapon_type, blueprint);
    }

    // Function to send the parts list to the `craft.rs` plugin
    pub fn send_to_craft_plugin(&self, crafting_plugin: &CraftPlugin, weapon_id: u32) -> Result<WeaponEntry, String> {
        if let Some(blueprint) = self.blueprints.weapons.get(&weapon_id) {
            let result = crafting_plugin.craft_weapon(blueprint, &self.parts);
            match result {
                Ok(crafted_weapon) => Ok(crafted_weapon),
                Err(error) => Err(error),
            }
        } else {
            Err(String::from("Weapon blueprint not found."))
        }
    }

    // Add a part to the parts list
    pub fn add_part(&mut self, part: WeaponPart) {
        self.parts.push(part);
    }

    // Clear parts after crafting is done
    pub fn clear_parts(&mut self) {
        self.parts.clear();
    }
}

// Represents a simple interface for the crafting plugin (in `craft.rs`)
pub struct CraftPlugin;

impl CraftPlugin {
    pub fn craft_weapon(&self, blueprint: &WeaponEntry, parts: &[WeaponPart]) -> Result<WeaponEntry, String> {
        // Check if all parts are available and match requirements
        if parts.len() < 3 { // Assume at least 3 parts needed for crafting
            return Err(String::from("Not enough parts."));
        }

        // Dummy success logic for crafting
        println!("Crafting weapon: {}", blueprint.name);
        Ok(blueprint.clone())
    }
}

// Define WeaponPart struct for crafting
#[derive(Clone)]
pub struct WeaponPart {
    pub name: String,
    pub quantity: u32,
}
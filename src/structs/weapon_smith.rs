use super::{WeaponsList, Ammo, WeaponTypes, WeaponPart};

#[derive(Default)]
pub struct WeaponSmith {

    // General weapon properties
    pub weapon_id: u32, // Reference the unique ID from WeaponsList
    pub ammo: Option<Ammo>,
    pub weapon_types: Vec<WeaponTypes>,
    pub weapons_list: Option<WeaponsList>,
    pub level_requirement: i32, // Minimum level required to use the weapon


    // Weapon parts
    pub parts: Vec<WeaponPart>, // Holds parts needed for crafting or modifying this weapon
}

impl WeaponSmith {
    pub fn new(weapon_id: u32) -> Self {
        WeaponSmith {
            weapon_id,
            ammo: None,
            weapon_types: vec![],
            weapons_list: None,
            level_requirement: 1,
            parts: vec![], // Initialize an empty parts list
        }
    }

    
    // Example method to add a part to the weapon
    pub fn add_part(&mut self, part: WeaponPart) {
        self.parts.push(part);
    }
}
use std::collections::HashMap;

#[derive(Clone)]
pub struct WeaponPart {
    pub id: u32, // Unique ID for each part
    pub name: String,
    pub description: String,
    pub quantity: u32, // Quantity available
}

pub struct WeaponPartsList {
    pub parts: HashMap<u32, WeaponPart>,
}

impl WeaponPartsList {
    pub fn new() -> Self {
        let mut parts = HashMap::new();
        // Example parts for crafting weapons
        parts.insert(1, WeaponPart {
            id: 1,
            name: String::from("AR Barrel"),
            description: String::from("A barrel used for making assault rifles."),
            quantity: 10,
        });
        parts.insert(2, WeaponPart {
            id: 2,
            name: String::from("Trigger Mechanism"),
            description: String::from("A mechanism used to trigger the firing mechanism."),
            quantity: 15,
        });
        parts.insert(3, WeaponPart {
            id: 3,
            name: String::from("Stock"),
            description: String::from("A gun stock to stabilize during firing."),
            quantity: 8,
        });
        parts.insert(4, WeaponPart {
            id: 4,
            name: String::from("Shotgun Barrel"),
            description: String::from("A barrel used specifically for shotguns."),
            quantity: 5,
        });
        parts.insert(5, WeaponPart {
            id: 5,
            name: String::from("Pump Mechanism"),
            description: String::from("A pump mechanism used for shotguns."),
            quantity: 6,
        });
        parts.insert(6, WeaponPart {
            id: 6,
            name: String::from("Recoil Pad"),
            description: String::from("A pad to reduce recoil when firing."),
            quantity: 7,
        });
        parts.insert(7, WeaponPart {
            id: 7,
            name: String::from("Scope"),
            description: String::from("A long-range optical scope used for sniper rifles."),
            quantity: 12,
        });
        parts.insert(8, WeaponPart {
            id: 8,
            name: String::from("Crossbow Limb"),
            description: String::from("The main limb used for crafting crossbows."),
            quantity: 4,
        });
        parts.insert(9, WeaponPart {
            id: 9,
            name: String::from("Bowstring"),
            description: String::from("A bowstring used for bows and crossbows."),
            quantity: 10,
        });
        parts.insert(10, WeaponPart {
            id: 10,
            name: String::from("Magic Gem"),
            description: String::from("A gem infused with magical properties for spell crafting."),
            quantity: 3,
        });
        parts.insert(11, WeaponPart {
            id: 11,
            name: String::from("Sword Blade"),
            description: String::from("A forged steel blade for swords."),
            quantity: 9,
        });
        parts.insert(12, WeaponPart {
            id: 12,
            name: String::from("Axe Head"),
            description: String::from("A heavy axe head used for crafting battle axes."),
            quantity: 7,
        });
        parts.insert(13, WeaponPart {
            id: 13,
            name: String::from("Handle"),
            description: String::from("A wooden or metal handle for melee weapons like hammers or axes."),
            quantity: 15,
        });
        parts.insert(14, WeaponPart {
            id: 14,
            name: String::from("Staff Core"),
            description: String::from("The core of a staff used for crafting magic weapons."),
            quantity: 5,
        });
        parts.insert(15, WeaponPart {
            id: 15,
            name: String::from("Fire Rune"),
            description: String::from("A rune containing fire magic, used for crafting fire spells."),
            quantity: 6,
        });
        parts.insert(16, WeaponPart {
            id: 16,
            name: String::from("Hammer Head"),
            description: String::from("A large hammer head used for crafting war hammers."),
            quantity: 5,
        });
        parts.insert(17, WeaponPart {
            id: 17,
            name: String::from("Flintlock Mechanism"),
            description: String::from("A mechanism used for making flintlock pistols."),
            quantity: 4,
        });
        parts.insert(18, WeaponPart {
            id: 18,
            name: String::from("Arrow Shaft"),
            description: String::from("A wooden shaft used to craft arrows for bows."),
            quantity: 20,
        });
        parts.insert(19, WeaponPart {
            id: 19,
            name: String::from("Feather Fletching"),
            description: String::from("Feathers used to stabilize arrows during flight."),
            quantity: 25,
        });
        parts.insert(20, WeaponPart {
            id: 20,
            name: String::from("Bolt Tip"),
            description: String::from("A metal tip used for crafting crossbow bolts."),
            quantity: 18,
        });
        parts.insert(21, WeaponPart {
            id: 21,
            name: String::from("Spell Focus Crystal"),
            description: String::from("A crystal used to enhance magical spells in staffs or wands."),
            quantity: 3,
        });
        parts.insert(22, WeaponPart {
            id: 22,
            name: String::from("Sword Guard"),
            description: String::from("A metal guard to protect the user's hand while wielding a sword."),
            quantity: 11,
        });
        parts.insert(23, WeaponPart {
            id: 23,
            name: String::from("Pistol Barrel"),
            description: String::from("A barrel used for crafting pistols."),
            quantity: 6,
        });

        WeaponPartsList { parts }
    }
}
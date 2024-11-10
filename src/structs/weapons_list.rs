// Define weapon constants using their unique ID

// Projectile based weapons
pub const ASSAULT_RIFLE: u32 = 1;  // p1
pub const SHOTGUN: u32 = 2;        // p2
pub const PISTOL: u32 = 3;         // p3
pub const SNIPER_RIFLE: u32 = 4;   // p4
pub const MACHINE_GUN: u32 = 17;   // p5
pub const ROCKET_LAUNCHER: u32 = 18; // p6

// Ranged weapons
pub const BOW: u32 = 5;            // r1
pub const CROSSBOW: u32 = 6;       // r2
pub const THROWING_KNIFE: u32 = 19; // r3
pub const SLINGSHOT: u32 = 20;     // r4

// Melee weapons
pub const SWORD: u32 = 7;          // m1
pub const HAMMER: u32 = 8;         // m2
pub const AXE: u32 = 9;            // m3
pub const SPEAR: u32 = 10;         // m4
pub const DAGGER: u32 = 11;        // m5
pub const FLAIL: u32 = 21;         // m6
pub const MACE: u32 = 22;          // m7

// Spells
pub const FIREBALL: u32 = 12;      // s1
pub const LIGHTNING_BOLT: u32 = 13;// s2
pub const ICE_SPIKE: u32 = 14;     // s3
pub const HEALING_AURA: u32 = 15;  // s4
pub const WIND_GUST: u32 = 16;     // s5
pub const EARTHQUAKE: u32 = 23;    // s6
pub const WATER_BARRIER: u32 = 24; // s7

// WeaponsList struct
use std::collections::HashMap;

pub struct WeaponsList {
    pub weapons: HashMap<u32, WeaponEntry>,
}

pub struct WeaponEntry {
    pub name: String,
    pub damage: i32,
    pub weapon_type: u32, // Reference the unique ID
    pub description: String,
    pub ammo_required: Option<u32>,
    pub parts_required: Vec<u32>, // New field to hold part IDs
    pub range: Option<i32>,
    pub weight: f32,
    pub durability: i32,
    pub elemental_type: Option<String>,
    pub critical_chance: f32,
    pub attack_speed: f32,
    pub rarity: String,
}

impl WeaponsList {
    pub fn new() -> Self {
        let mut weapons = HashMap::new();
        // Adding all weapons to the list

        // Projectile Based Weapons
        weapons.insert(ASSAULT_RIFLE, WeaponEntry {
            name: String::from("Assault Rifle"),
            damage: 30,
            weapon_type: ASSAULT_RIFLE,
            description: String::from("A basic assault rifle."),
            ammo_required: Some(5),
            parts_required: vec![1, 2, 3], // Example part IDs needed to craft this weapon
            range: Some(100),
            weight: 7.5,
            durability: 150,
            elemental_type: None,
            critical_chance: 5.0,
            attack_speed: 1.2,
            rarity: String::from("Common"),
        });

        weapons.insert(SHOTGUN, WeaponEntry {
            name: String::from("Shotgun"),
            damage: 40,
            weapon_type: SHOTGUN,
            description: String::from("A powerful close-range weapon."),
            ammo_required: Some(8),
            parts_required: vec![4, 5, 6],
            range: Some(50),
            weight: 8.5,
            durability: 140,
            elemental_type: None,
            critical_chance: 4.5,
            attack_speed: 0.9,
            rarity: String::from("Rare"),
        });

        weapons.insert(MACHINE_GUN, WeaponEntry {
            name: String::from("Machine Gun"),
            damage: 25,
            weapon_type: MACHINE_GUN,
            description: String::from("A rapid-fire machine gun."),
            ammo_required: Some(30),
            parts_required: vec![1, 2, 3, 8], // Example part IDs for crafting
            range: Some(120),
            weight: 10.0,
            durability: 180,
            elemental_type: None,
            critical_chance: 3.0,
            attack_speed: 2.5,
            rarity: String::from("Uncommon"),
        });

        // Melee Weapons
        weapons.insert(SWORD, WeaponEntry {
            name: String::from("Sword"),
            damage: 20,
            weapon_type: SWORD,
            description: String::from("A sharp steel sword, ideal for close combat."),
            ammo_required: None,
            parts_required: vec![11, 13, 22],
            range: Some(5),
            weight: 5.5,
            durability: 200,
            elemental_type: None,
            critical_chance: 4.0,
            attack_speed: 1.5,
            rarity: String::from("Common"),
        });

        weapons.insert(FLAIL, WeaponEntry {
            name: String::from("Flail"),
            damage: 35,
            weapon_type: FLAIL,
            description: String::from("A spiked flail for delivering heavy blows."),
            ammo_required: None,
            parts_required: vec![16, 18],
            range: Some(4),
            weight: 11.0,
            durability: 160,
            elemental_type: None,
            critical_chance: 5.5,
            attack_speed: 0.8,
            rarity: String::from("Rare"),
        });

        // Spells
        weapons.insert(FIREBALL, WeaponEntry {
            name: String::from("Fireball Spell"),
            damage: 45,
            weapon_type: FIREBALL,
            description: String::from("A fire spell that engulfs enemies in flames."),
            ammo_required: None,
            parts_required: vec![10, 15], // Magic Gem, Fire Rune
            range: Some(30),
            weight: 2.0,
            durability: 100,
            elemental_type: Some(String::from("Fire")),
            critical_chance: 8.0,
            attack_speed: 1.0,
            rarity: String::from("Rare"),
        });

        weapons.insert(EARTHQUAKE, WeaponEntry {
            name: String::from("Earthquake Spell"),
            damage: 60,
            weapon_type: EARTHQUAKE,
            description: String::from("A spell that sends shockwaves through the ground."),
            ammo_required: None,
            parts_required: vec![20, 25], // Earth Rune, Magic Core
            range: Some(15),
            weight: 3.0,
            durability: 90,
            elemental_type: Some(String::from("Earth")),
            critical_chance: 6.0,
            attack_speed: 0.7,
            rarity: String::from("Epic"),
        });

        WeaponsList { weapons }
    }
}
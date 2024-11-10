// Importing necessary types and modules
use horizon_data_types::Player;  // Data types related to players in the game
use socketioxide::extract::SocketRef;  // A reference to the WebSocket connection
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};  // Core types from the Horizon Plugin API
use std::sync::{RwLock, Arc};  // Thread-safe reference counting and read-write locking
use std::collections::HashMap; // HashMap for managing plugins

mod structs;

pub use structs::*;
use structs::weapon_smith::WeaponSmith;
use structs::ammo::Ammo;
use structs::weapontypes::WeaponTypes;
use structs::weaponslist::WeaponsList;
use structs::player_inventory::PlayerInventory;
use structs::weapon_parts::WeaponPart;
use structs::item::WeaponItem;
use structs::crafter::WeaponCrafter;
use structs::storage_container::StorageContainer;
use structs::weapon_blueprint::WeaponBlueprint;  // Changed from CraftableWeapon to WeaponBlueprint for clarity and future ease of useage with that of craft.rs and inventory.rs plugins still in development.
use structs::gun_catalog::GunCatalog;

// --- Plugin API Definition ---
// Define a trait called `Plugin_API` which other plugins can implement.
// This trait defines the methods that each plugin must provide.
pub trait Plugin_API {
    // Method that returns a string, can be used for identifying the plugin or simple functionality
    fn thing(&self) -> String;

    // Method that handles player joining the server
    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<Player>>>);

    // Method to handle crafting interactions
    fn handle_crafting_request(&self, item_id: u32) -> String;
}

// Defines a trait for plugin construction
pub trait Plugin_Construct {
    // Constructor method for initializing the plugin.
    // Takes a HashMap of loaded plugins and returns a new Plugin instance.
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin;
}

// --- Plugin Construction Implementation ---
// Implement the `Plugin_Construct` trait for the `Plugin` struct.
// This function initializes the plugin, providing access to other loaded plugins if needed.
impl Plugin_Construct for Plugin {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin {
        // Initialize the WeaponsList
        let mut weapons_list = WeaponsList::new();
        
        // Example: Adding a new weapon using its ID
        let weapon_id = ASSAULT_RIFLE;
        if let Some(weapon) = weapons_list.weapons.get(&weapon_id) {
            println!("Weapon added: {} with damage: {}", weapon.name, weapon.damage);
        }
        
        // Simple log message to confirm plugin initialization
        println!("Weapon_Smith Plugin Has Been Fully Initialized.");
        
        // Example: Access inventory or skills script plugins if available
        if let Some(inventory_plugin) = plugins.get("inventory_plugin") {
            println!("Inventory plugin is available.");
        }
        if let Some(skills_plugin) = plugins.get("skillscript") {
            println!("SkillScript plugin is available.");
        }
        
        // Return a new instance of the Plugin struct
        Plugin {}
    }
}

// --- Plugin API Implementation ---

// Implement the `Plugin_API` trait for the `Plugin` struct
impl Plugin_API for Plugin {
    // Implementation of the `thing` method, which returns a custom string
    fn thing(&self) -> String {
        println!("this is a simple print string but fire whatever events you want in its place!");
        // Trigger an actual event here, for instance sending a message to connected clients or broadcasting a game event
        let event = GameEvent::CustomEvent("WeaponSmithActive".to_string(), HashMap::new());
        // Assuming we have access to a game engine plugin or similar
        if let Some(unreal_plugin) = UNREAL_ENGINE_PLUGIN.get() {
            unreal_plugin.on_game_event(&event);
        }
        "this is a simple print string but fire whatever events you want in its place!".to_string()
    }

    // Implementation of the `player_joined` method, triggered when a player joins the server.
    // It sets up WebSocket listeners for handling events related to the player.
    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<Player>>>) {
        // Call the function to set up event listeners for this player
        setup_listeners(socket, players);
    }

    // Implementation of the crafting request handler, returns crafting results or status
    fn handle_crafting_request(&self, item_id: u32) -> String {
        println!("Crafting request received for item ID: {}", item_id);
        // Here we simulate crafting logic by checking if the item exists in the weapons list
        let weapons_list = WeaponsList::new(); // Initialize weapons list
        if let Some(weapon) = weapons_list.weapons.get(&item_id) {
            return format!("Crafted: {} with damage: {}", weapon.name, weapon.damage);
        }
        "Item not found in Weapons List.".to_string()
    }
}

// --- Event Listener ---
// Function to set up WebSocket event listeners.
// Here, it listens for the event "foo" and logs "bar" when the event occurs.
fn setup_listeners(socket: SocketRef, players: Arc<RwLock<Vec<Player>>>) {
    // Example event listener for the "foo" event
    socket.on("foo", || println!("bar"));
}
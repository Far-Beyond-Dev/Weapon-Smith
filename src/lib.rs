use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use log::{warn, debug, info};

mod structs;
use structs::*;

// Define plugin-specific type aliases
#[cfg(feature = "unreal_engine_horizon")]
pub mod plugin_types {
    pub type UnrealPluginAPI = dyn super::PluginAPI;
}

#[macro_export]
macro_rules! get_plugin_safe {
    ($name:ident, $plugins:expr) => {{
        #[cfg(feature = stringify!($name))]
        {
            use crate::plugin_types;
            let plugin_result = $plugins
                .get(stringify!($name))
                .map(|p| &p.instance as &plugin_types::UnrealPluginAPI);
            
            if plugin_result.is_none() {
                log::warn!("Plugin {} not found - this feature will be skipped", stringify!($name));
            }
            
            plugin_result
        }
        
        #[cfg(not(feature = stringify!($name)))]
        {
            log::debug!("Plugin {} is not enabled in this build", stringify!($name));
            None::<&dyn PluginAPI>
        }
    }};
}

pub trait Plugin_API {
    fn thing(&self) -> String;
    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<Player>>>);
    fn handle_crafting_request(&self, item_id: u32) -> String;
}

pub trait Plugin_Construct {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin;
}

impl Plugin_Construct for Plugin {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin {
        let mut weapons_list = weapons_list::WeaponsList::new();
        
        let weapon_id = weapons_list::ASSAULT_RIFLE;
        if let Some(weapon) = weapons_list.weapons.get(&weapon_id) {
            info!("Weapon added: {} with damage: {}", weapon.name, weapon.damage);
        }

        // Handle Unreal Engine integration with feature gate
        #[cfg(feature = "unreal_engine_horizon")]
        {
            if let Some(unreal_plugin) = get_plugin_safe!(unreal_engine_horizon, plugins) {
                debug!("Unreal Engine Horizon plugin found and enabled");
                if let Some(event) = GameEvent::new() {
                    unreal_plugin.on_game_event(&event);
                }
            }
        }

        info!("Weapon_Smith Plugin Has Been Fully Initialized.");
        
        // Feature-gated plugin checks
        #[cfg(feature = "inventory_plugin")]
        if let Some(inventory_plugin) = plugins.get("inventory_plugin") {
            info!("Inventory plugin is available and enabled.");
        }

        #[cfg(feature = "skillscript")]
        if let Some(skills_plugin) = plugins.get("skillscript") {
            info!("SkillScript plugin is available and enabled.");
        }
        
        Plugin {}
    }
}

impl Plugin_API for Plugin {
    fn thing(&self) -> String {
        debug!("Executing thing() method");
        "this is a simple print string but fire whatever events you want in its place!".to_string()
    }

    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<Player>>>) {
        setup_listeners(socket, players);
    }

    fn handle_crafting_request(&self, item_id: u32) -> String {
        info!("Crafting request received for item ID: {}", item_id);
        let weapons_list = weapons_list::WeaponsList::new();
        if let Some(weapon) = weapons_list.weapons.get(&item_id) {
            return format!("Crafted: {} with damage: {}", weapon.name, weapon.damage);
        }
        "Item not found in Weapons List.".to_string()
    }
}

fn setup_listeners(socket: SocketRef, players: Arc<RwLock<Vec<Player>>>) {
    socket.on("foo", || debug!("bar"));
}
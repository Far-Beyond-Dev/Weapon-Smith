# Weapon_Smith

### Introduction

The `Weapon_Smith` system is a server-side plugin library built for the `Horizon-Community-Edition` game server. Implemented in Rust, it provides comprehensive weapon-related functionality including weapon crafting, part assembly, and weapon management. The plugin can either integrate with the official Horizon inventory plugin or use its own built-in inventory system via feature flags. It is designed to be used by a custom server-side game plugin, which handles the actual game logic and client interactions.

### Installation

1. Drop the plugin into your Horizon server's `plugins` directory:
```bash
Horizon-Community-Edition/
└── plugins/
    └── weapon_smith/
```

2. Horizon will automatically code-generate the necessary import file to load the plugin with default settings.

3. To enable the built-in inventory system, modify the plugin's `Cargo.toml` to include the feature in default features:

```toml
[package]
name = "weapon_smith"
version = "0.1.0"

[features]
default = ["inventory_plugin"]  # Enable built-in inventory by default
inventory_plugin = []           # Built-in inventory system
unreal_engine_horizon = []      # Reserved for future use
skillscript = []                # Reserved for future use
```

### Architecture Overview

The Weapon_Smith plugin operates as a library that:
- Can either:
  - Integrate with the official Horizon inventory plugin for item storage and management, or
  - Use its own built-in inventory system when enabled via the `inventory_plugin` feature flag
- Provides weapon crafting and management capabilities to the server-side game plugin
- Functions entirely server-side, with no direct client communication
- Maintains weapon definitions, parts lists, and crafting logic

### Directory Structure

```
Horizon-Community-Edition/
└── plugins/
    └── weapon_smith/
        ├── src/
        │   ├── lib.rs                 # Plugin library entry point and feature handling
        │   ├── mod.rs                 # Module definitions
        │   ├── weapons_list.rs        # Weapon definitions and properties
        │   ├── weapon_parts_list.rs   # Weapon parts catalog
        │   ├── weapon_smith.rs        # Core weapon management logic
        │   ├── storage_container.rs   # Storage container implementation
        │   ├── player_inventory.rs    # Built-in inventory implementation
        │   └── item.rs                # Base weapon item definitions
        └── Cargo.toml                 # Plugin manifest and feature configuration
```

### Feature Configuration

The plugin's features are configured through its `Cargo.toml` file. The built-in inventory system can be enabled by including `inventory_plugin` in the default features:

```toml
[features]
default = ["inventory_plugin"]  # Enable built-in inventory by default
inventory_plugin = []          # Built-in inventory system
```

### Key Components

**1. Core Plugin Integration (`lib.rs`)**
- Implements the Plugin_API trait for integration with the main game server
- Provides handlers for crafting requests and player events
- Manages feature flag configurations and plugin initialization

**2. Inventory Systems**

The plugin offers two inventory management approaches:

a) **Built-in Inventory System** (enabled via `inventory_plugin` feature)
- Implemented in `player_inventory.rs` and `storage_container.rs`
- Provides a complete inventory management system
- Features:
  - Fixed-slot inventory system
  - Storage containers with UUID tracking
  - Item management (add, remove, retrieve)
  ```rust
  #[cfg(feature = "inventory_plugin")]
  let inventory = PlayerInventory::new(30);
  ```

b) **Horizon Inventory Integration**
- Uses the official Horizon inventory plugin
- Active when `inventory_plugin` feature is not enabled

**3. Weapon System (`weapons_list.rs`, `weapon_parts_list.rs`)**
- Defines all available weapons and their properties
- Maintains a catalog of weapon parts used in crafting
- Categories include:
  - Projectile weapons (Assault Rifle, Shotgun, etc.)
  - Ranged weapons (Bow, Crossbow)
  - Melee weapons (Sword, Hammer)
  - Spell weapons (Fireball, Lightning Bolt)

**4. Item Management (`item.rs`)**
- Defines the base WeaponItem structure
- Handles weapon properties and attributes
- Provides methods for item modification

### Usage

The plugin is automatically loaded by Horizon's plugin system. The server-side game plugin can interact with it through the Plugin_API:

```rust
// In the server-side game plugin

let weapon_smith = getplugin!(weapon_smith, plugins)

fn handle_crafting_request(&self, item_id: u32) -> String {
  weapon_smith.handle_crafting_request(item_id)
}
```

### Plugin Dependencies

When using the default configuration (no built-in inventory):
- **Recommended**: Horizon Inventory Plugin
  - Must be present in the plugins directory
  - Will be loaded automatically by Horizon's plugin system and auto-detected

When using built-in inventory (`inventory_plugin` feature enabled):
- No additional plugin dependencies required

### Notes for Developers

- This is a server-side library - all client communication should be handled by the game plugin
- The plugin does not directly interact with clients
- Feature flags are configured in the plugin's own `Cargo.toml`
- Weapon definitions and crafting logic can be extended through the provided structures
- The plugin is loaded automatically by Horizon's plugin system

### Contributing

Contributions should focus on:
- Enhancing server-side functionality
- Improving both built-in and Horizon inventory integration
- Adding new weapon types and crafting recipes
- Optimizing performance and resource usage
- Extending feature flag functionality
- Expanding the API to allow for more complex operations such as server-side hit calculation

### License

Apache 2.0
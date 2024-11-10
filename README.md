# Weapon_Smith
 
### Introduction

The `Weapon_Smith` system is a new and advanced modular plugin built for the `Horizon-Community-Edition` game server. It is implemented using Rust and is designed to facilitate a wide range of weapon-related functionality, including weapon crafting, inventory management, part assembly, and storage. This plugin interacts seamlessly with an Unreal Engine 5.4+ client and a PebbleVault spatial database to create a sophisticated in-game experience for both players and developers.

### Directory Structure

The `Weapon_Smith` plugin is structured under the `Horizon-Community-Edition` and owned by horizon until otherwise given notice as follows however for now its owned by them so dont try to steal this we will find you and throw rocks at you lol.:

```
Horizon-Community-Edition/
├── Weapon_Smith/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── structs/
│   │   │   ├── weapon_smith.rs
│   │   │   ├── weapon_parts.rs
│   │   │   ├── weapon_list.rs
│   │   │   ├── weapon_bench.rs
│   │   │   ├── player_inventory.rs
│   │   │   ├── storage_container.rs
│   │   │   ├── item.rs
│   │   ├── crafting/
│   │   │   ├── craft.rs
│   │   ├── tests/
│   │   │   ├── tests.rs
├── README.md
```

### Detailed Changes and New Features

**1. Weapon_Smith Plugin Design and Core Functionality**

The `Weapon_Smith` plugin has been built from scratch to provide a more comprehensive, flexible, and expandable weapon management system for the game. The main changes are as follows:

- **WeaponSmith Struct**: 
  - The `WeaponSmith` struct is the heart of the plugin. It represents the fundamental details about any weapon within the game world. This struct includes properties like `weapon_id`, `ammo`, `weapon_types`, `level_requirement`, and `weapons_list` to maintain a connection with the complete inventory.
  - The `weapon_id` field provides a unique identifier for each weapon based on data from the `weapon_list.rs`.

**2. WeaponsList and WeaponEntry Structures**

- **WeaponsList (`weapon_list.rs`)**:
  - The `WeaponsList` struct stores a collection of all available weapons, each represented as a `WeaponEntry`.
  - Added fields like `ammo_required`, `parts_required`, `elemental_type`, `critical_chance`, `attack_speed`, and `rarity` to capture a more detailed definition of weapons.
  - The list includes multiple weapon categories, such as projectile-based weapons (e.g., `ASSAULT_RIFLE`), ranged weapons (e.g., `BOW`), melee weapons (e.g., `SWORD`), and spells (e.g., `FIREBALL`).

- **WeaponEntry Struct**:
  - This struct defines the properties for each individual weapon within the `WeaponsList`. It holds weapon-specific details such as `damage`, `ammo_required`, `parts_required` (list of `WeaponPart` IDs required for crafting), `range`, `weight`, and `durability`.

**3. WeaponParts and Crafting Bench**

- **WeaponParts (`weapon_parts.rs`)**:
  - The `WeaponPartsList` struct contains various parts used to craft weapons. Each part, represented as a `WeaponPart`, has a unique ID (`part_id`), `name`, `description`, and `quantity`.
  - Example parts include `AR Barrel`, `Trigger Mechanism`, `Stock`, etc. These parts are assigned unique IDs, making them easily identifiable during crafting.

- **WeaponCraftingBench (`weapon_bench.rs`)**:
  - The `WeaponCraftingBench` struct handles the actual crafting operations using weapon parts.
  - The bench allows adding blueprints and parts, sending crafting requests to the `CraftPlugin` (`craft.rs`). This modular approach lets developers decide whether to implement the crafting mechanism within the plugin or connect to a broader crafting system.

**4. Player Inventory and Storage**

- **Player Inventory (`player_inventory.rs`)**:
  - The `PlayerInventory` struct uses a HashMap to store items in fixed slots. Items are stored as `WeaponItem` structs, containing properties like `id`, `name`, `description`, `weight`, and `value`.
  - Functions are provided for adding, removing, and managing items in the player's inventory.

- **Storage Container (`storage_container.rs`)**:
  - The `StorageContainer` struct represents in-game storage for weapons and parts, enabling players to stash items for later use.
  - The storage uses a UUID (`uuid`) to ensure unique identification. The container itself stores a `PlayerInventory` instance that keeps track of weapon items.

### Interactions Between Components

**1. Crafting and Weapon Assembly**

- Weapons can be crafted using `WeaponParts` combined at the `WeaponCraftingBench`. For example, crafting an `ASSAULT_RIFLE` involves combining parts like an `AR Barrel`, `Trigger Mechanism`, and `Stock`.
- The crafting logic involves sending crafting requests to the `CraftPlugin`, where the plugin validates whether all necessary parts are present, and then successfully crafts the item.

**2. Inventory Management**

- Crafted weapons can be added to the player's inventory (`PlayerInventory`). The inventory maintains a list of `WeaponItems` stored in individual slots, each represented by a unique ID and containing additional metadata (e.g., `weight`, `description`, `rarity`).
- Weapons and parts can also be stored in a `StorageContainer`. This container allows players to maintain a separate stash outside of their regular inventory, useful for managing rare weapons or for organizing parts efficiently.

### Example Workflow

**1. Crafting a Weapon**

- A player finds or buys different `WeaponParts`.
- These parts are placed in the `WeaponCraftingBench`, which allows the player to select a weapon blueprint (`WeaponEntry` from `WeaponsList`) to craft.
- The crafting request is processed by `CraftPlugin`, which ensures all required parts are available.
- Upon successful crafting, the completed weapon (`WeaponEntry`) is added to the player's inventory (`PlayerInventory`).

**2. Storing and Retrieving Weapons**

- Players can move their weapons to a `StorageContainer` when they need to free up inventory space or want to organize weapons separately.
- The `StorageContainer` uses a `PlayerInventory` system to keep track of its contents. Players can add, retrieve, or remove items as needed.

### Summary of New Features

1. **Weapon Crafting System**:
   - Introduced a crafting system that utilizes weapon parts (`WeaponPartsList`) to build complete weapons. Players can combine parts to create powerful, fully functional weapons.

2. **Weapon Parts and Assembly**:
   - Added `WeaponParts` and a crafting bench (`WeaponCraftingBench`) to handle the detailed assembly process of crafting weapons.

3. **Enhanced Weapon Definitions**:
   - Weapons are now defined with rich properties, including damage, range, elemental types, attack speed, rarity, and critical hit chance.

4. **Inventory and Storage Systems**:
   - Implemented a `PlayerInventory` and `StorageContainer` system, enabling flexible weapon storage, easy item management, and organized separation between active inventory and stash storage.

### Full Directory Tree with Explanations

```
Horizon-Community-Edition/
├── Weapon_Smith/
│   ├── src/
│   │   ├── lib.rs  # Main library entry point for Weapon_Smith plugin.
│   │   ├── structs/
│   │   │   ├── weapon_smith.rs  # Main struct for weapon definitions and core properties.
│   │   │   ├── weapon_parts.rs  # List of all weapon parts that can be used in crafting.
│   │   │   ├── weapon_list.rs  # Complete list of weapons available, including properties and parts required.
│   │   │   ├── weapon_bench.rs  # Handles weapon crafting using parts.
│   │   │   ├── player_inventory.rs  # Manages player's inventory of weapons and items.
│   │   │   ├── storage_container.rs  # Represents in-game storage for weapons, parts, and inventory.
│   │   │   ├── item.rs  # Basic weapon item structure, containing essential attributes like id, name, and value.
│   │   ├── crafting/
│   │   │   ├── craft.rs  # CraftPlugin responsible for handling crafting logic and processing crafting requests.
│   │   ├── tests/
│   │   │   ├── tests.rs  # Testing module for Weapon_Smith functionality.
├── README.md  # Documentation file containing detailed descriptions of all changes and features.
```

### Conclusion

The `Weapon_Smith` plugin introduces a powerful new modular system for weapon creation, management, and integration in the Horizon-Community-Edition game server. With well-defined components for crafting, parts management, weapon definitions, and player inventories, it offers a robust foundation for building complex and engaging weapon-related gameplay. Its modular design ensures that each part of the system is loosely coupled, facilitating easy updates, extension, or replacement.

This documentation has covered all new features, design considerations, and interactions among the different components within `Weapon_Smith`, providing a deep understanding of its functionality and future potential. All Developers are Welcomed and Fully encouraged to extend these capabilities further, using the plugin's design to build engaging and immersive experiences.
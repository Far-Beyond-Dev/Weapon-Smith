#[derive(Clone)]
pub struct WeaponItem {
    pub id: u32, // Unique identifier for the item
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub value: u32, // Monetary or crafting value of the item
}

impl WeaponItem {
    pub fn new(id: u32, name: &str, description: &str, weight: f32, value: u32) -> Self {
        WeaponItem {
            id,
            name: String::from(name),
            description: String::from(description),
            weight,
            value,
        }
    }

    // Example method to increase the value of the weapon
    pub fn increase_value(&mut self, amount: u32) {
        self.value += amount;
    }

    // Example method to modify the weight of the weapon
    pub fn modify_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}
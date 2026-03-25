use serde::{Deserialize, Serialize};
use chrono::{Local, Timelike};

// Defines the type of animal to determine behavior patterns
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Species {
    Cat,
    Dog,
    Penguin,
}

// Defines the rarity scale for the trading system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

// Main structure representing a desktop pet
#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
    pub pet_id: String,
    pub species: Species,
    pub rarity_level: Rarity,
    pub growth_stage: String,
    pub current_energy: u8,
    pub is_sleeping: bool,
    pub position_x: i32,
    pub position_y: i32,
    pub security_hash: String,
}

impl Pet {
    // Calculates sleep state based on system local time and species
    pub fn update_circadian_rhythm(&mut self) {
        let current_hour = Local::now().hour();
        
        self.is_sleeping = match self.species {
            Species::Cat => {
                // Cats are nocturnal: they sleep during the day (8:00 to 18:00)
                current_hour >= 8 && current_hour < 18
            },
            _ => {
                // Diurnal animals: they sleep at night (22:00 to 06:00)
                current_hour >= 22 || current_hour < 6
            }
        };
    }

    // Decreases energy safely without underflowing below 0
    pub fn consume_energy(&mut self, amount: u8) {
        if self.current_energy > amount {
            self.current_energy -= amount;
        } else {
            self.current_energy = 0;
        }
    }
}
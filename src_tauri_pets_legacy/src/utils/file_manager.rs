use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
// Assuming the pet model is accessible at crate::models::pet::Pet
use crate::models::pet::Pet;
use serde_json;

const STORAGE_PATH: &str = "./local_storage/pets_data.json";
const STORAGE_DIR: &str = "./local_storage";

pub struct FileManager;

impl FileManager {
    // Verifies and creates the directory structure if it is missing
    fn ensure_directory_exists() -> std::io::Result<()> {
        let path = Path::new(STORAGE_DIR);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    // Serializes a vector of pets and writes them safely to disk
    pub fn save_pets(pets: &Vec<Pet>) -> Result<(), String> {
        Self::ensure_directory_exists().map_err(|e| e.to_string())?;

        // Using pretty print for easier debugging of the JSON file
        let serialized_data = serde_json::to_string_pretty(pets)
            .map_err(|e| format!("Serialization error: {}", e))?;

        let mut file = File::create(STORAGE_PATH)
            .map_err(|e| format!("Failed to create storage file: {}", e))?;

        file.write_all(serialized_data.as_bytes())
            .map_err(|e| format!("Failed to write data to disk: {}", e))?;

        Ok(())
    }

    // Reads the JSON file and reconstructs the Pet objects in memory
    pub fn load_pets() -> Result<Vec<Pet>, String> {
        if !Path::new(STORAGE_PATH).exists() {
            // Returns an empty collection if this is the first time running the app
            return Ok(Vec::new());
        }

        let mut file = File::open(STORAGE_PATH)
            .map_err(|e| format!("Failed to open storage file: {}", e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read file contents: {}", e))?;

        let pets: Vec<Pet> = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        Ok(pets)
    }
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod services;
mod utils;

use std::sync::Mutex;
use tauri::State;

use models::pet::Pet;
use services::system_monitor::{HardwareMonitor, SystemResources};
use utils::file_manager::FileManager;

// Retrieves the current system resources (CPU and RAM) to be sent to the frontend
#[tauri::command]
fn get_system_resources(monitor_state: State<'_, Mutex<HardwareMonitor>>) -> Result<SystemResources, String> {
    let mut monitor = monitor_state
        .lock()
        .map_err(|_| "Failed to acquire lock on hardware monitor")?;
    
    Ok(monitor.get_current_resources())
}

// Reads the pets.json file and returns the list of pets to the frontend
#[tauri::command]
fn load_pets() -> Result<Vec<Pet>, String> {
    FileManager::load_pets()
}

// Receives a vector of pets from the frontend and overwrites the pets.json file
#[tauri::command]
fn save_pets(pets: Vec<Pet>) -> Result<(), String> {
    FileManager::save_pets(&pets)
}

fn main() {
    // Initialize the hardware monitor wrapped in a Mutex for thread-safe state management
    let hardware_monitor = Mutex::new(HardwareMonitor::new());

    tauri::Builder::default()
        // Register the hardware monitor as a managed state
        .manage(hardware_monitor)
        // Register the functions that will be callable from the frontend
        .invoke_handler(tauri::generate_handler![
            get_system_resources,
            load_pets,
            save_pets
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
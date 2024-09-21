// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod note;

#[tauri::command]
fn get_notes(search: String) -> Result<Vec<String>, String> {
  return note::get_notes(search);
}

#[tauri::command]
fn create_note(name: &str) -> Result<(), String> {
  return note::create_note(name);
}

#[tauri::command]
fn save_note(name_old: &str, name_new: &str, content: &str) -> Result<(), String> {
  return note::save_note(name_old, name_new, content);
}

#[tauri::command]
fn read_note(name: &str) -> Result<String, String> {
  return note::read_note(name);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_notes,
      create_note,
      read_note,
      save_note
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

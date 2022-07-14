#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};


fn main() {
  
  let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
  let close = CustomMenuItem::new("Close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit));
  let menu = Menu::new().add_native_item(MenuItem::Copy).add_item(CustomMenuItem::new("Hide","Hide")).add_submenu(submenu);
  tauri::Builder::default()
  .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

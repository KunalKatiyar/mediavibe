#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

use tauri::{CustomMenuItem, Menu, Submenu};

fn main() {
    // let file = CustomMenuItem::new("OpenFile".to_string(), "Open File");
    // let youtube = CustomMenuItem::new("Youtube".to_string(), "Youtube");
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let submenu = Submenu::new(
        "File",
        Menu::new().add_item(quit),
    );
    let menu = Menu::new().add_submenu(submenu);
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            // "OpenFile" => {
            //     println!("{:?}", event);
            //     event.
            // }
            // "Youtube" => {
            //     event.window().close().unwrap();
            // }
            "Quit" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

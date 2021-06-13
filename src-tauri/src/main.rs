#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::vec::Vec;
use tauri::{Menu, MenuItem};

// Definition in main.rs

struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  other_val: usize,
}

async fn some_other_function() -> Option<String> {
  Some("response".into())
}

#[tauri::command]
async fn my_custom_command(
  window: tauri::Window,
  number: usize,
  _database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
  println!("Called from {}", window.label());
  let result: Option<String> = some_other_function().await;
  if let Some(message) = result {
    Ok(CustomResponse {
      message,
      other_val: 42 + number,
    })
  } else {
    Err("No result".into())
  }
}

fn uri_handler(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
  println!("tune:// uri handler called with {}", data);
/*
  let mut bytes = fs::read("/Users/freik/razor.flac").expect("File not found");
  let mut hdr = String::new();
  write!(&mut hdr, "HTTP/1.1 200 OK\r\n").unwrap();
  write!(&mut hdr, "Content-Length: {}\r\n", bytes.len()).unwrap();
  write!(&mut hdr, "Content-Type: audio/x-flac\r\n").unwrap();
  write!(&mut hdr, "Connection: Close\r\n").unwrap();
  write!(&mut hdr, "\r\n").unwrap();
  println!("Read in {} bytes", bytes.len());
  print!("{}", hdr);
  println!("{}, {}, {}", tmp[0], tmp[1], tmp[2]);
  res.append(&mut hdr.into_bytes());
  */
  let mut info = String::new();
  write!(&mut info, "<html><body><div>Hello</div></body></html>").unwrap();
  let mut res = Vec::new();
  res.append(&mut info.into_bytes());
  Ok(res)
}

fn main() {
  let menu = vec![
    // on macOS first menu is always app name
    Menu::new(
      "RuMP",
      vec![
        MenuItem::Services,
        MenuItem::Separator,
        MenuItem::Hide,
        MenuItem::HideOthers,
        MenuItem::ShowAll,
        MenuItem::Separator,
        MenuItem::Quit,
        MenuItem::CloseWindow,
      ],
    ),
    Menu::new(
      "Edit",
      vec![
        MenuItem::Undo,
        MenuItem::Redo,
        MenuItem::Separator,
        MenuItem::Cut,
        MenuItem::Copy,
        MenuItem::Paste,
        MenuItem::Separator,
        MenuItem::SelectAll,
      ],
    ),
  ];
  tauri::Builder::default()
    .manage(Database {})
    .menu(menu)
    .register_global_uri_scheme_protocol("tune", &uri_handler)
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {      
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            cmd::Cmd::PrintOnRustSync { 
              payload 
            } => cmd::print_on_rust_sync(payload),
            cmd::Cmd::GenerateRandomAsync { 
              payload, 
              callback, 
              error 
            } => cmd::generate_random_async(_webview, payload, callback, error)
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}

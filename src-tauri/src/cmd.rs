use serde::{ Deserialize, Serialize };
use tauri::Webview;
use rand::Rng;

// Command Error
#[derive(Debug, Clone)]
pub struct Error<'a> {
  message: &'a str
}

impl<'a> Error<'a> {
  pub fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for Error<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> std::error::Error for Error<'a> {}

#[derive(Serialize)]
pub struct Response<'a> {
  pub value: String,
  pub message: &'a str,
}

// Commands
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  PrintOnRustSync { 
    payload: String 
  },
  GenerateRandomAsync { 
    payload: String,
    error: String,
    callback: String
  }
}

// Generate Random Async
pub fn generate_random_async(_webview: &mut Webview<'_> , payload: String, callback: String, error: String) {
  

  tauri::execute_promise(
    _webview,
    move || {
      let mut rng = rand::thread_rng();

      match payload.as_str() {
       "u8" => {
          let value: u8 = rng.gen();

          Ok(Response {
            value: format!("{}", value),
            message: "Value loaded"
          })
        },
      "u16" => {
          let value: u16 = rng.gen();

          Ok(Response {
            value: format!("{}", value),
            message: "Value loaded"
          })
        },
       "u32" => {
          let value: u32 = rng.gen();

          Ok(Response {
            value: format!("{}", value),
            message: "Value loaded"
          })
        },
        "u64" => {
          let value: u64 = rng.gen();

          Ok(Response {
            value: format!("{}", value),
            message: "Value loaded"
          })
        },
        "u128" => {
          let value: u128 = rng.gen();

          Ok(Response {
            value: format!("{}", value),
            message: "Value loaded"
          })
        },
        _ => Err(Error::new("Select num type").into())
      }
    },
    callback,
    error,
  )
}

// Print on Rust Sync
pub fn print_on_rust_sync(payload: String) {
  println!("{}", payload);
}
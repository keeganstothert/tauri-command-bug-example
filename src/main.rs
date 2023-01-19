#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::net::TcpStream;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const IP: &str = "127.0.0.1:8731";

#[tauri::command]
async fn get_connection() -> Result<(), Error> {
    let tcp_stream = match TcpStream::connect("127.0.0.1:8700").await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to desktop stream");
            return Err(Error::from(e));
        }
    };

    return Ok(());
}

use serde::{ser::Serializer, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    Tauri(#[from] tauri::Error),
    Utf8(#[from] std::str::Utf8Error),
    Generic(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self)
    }
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::net::TcpStream;
use utilities::AppError;

mod utilities;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const IP: &str = "127.0.0.1:8731";

#[tauri::command]
async fn get_connection() -> Result<bool, AppError> {
    let tcp_stream = match TcpStream::connect(IP).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to desktop stream");
            return Err(e);
        }
    };

    return Ok(true);
}

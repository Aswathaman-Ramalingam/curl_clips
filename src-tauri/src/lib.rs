use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct YtDlpFormat {
    format_id: String,
    format_note: Option<String>,
    ext: String,
    vcodec: Option<String>,
    acodec: Option<String>,
    filesize: Option<i64>,
    height: Option<i32>,
    width: Option<i32>,
    fps: Option<f64>,
    tbr: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct YtDlpResponse {
    id: String,
    title: String,
    formats: Vec<YtDlpFormat>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn fetch_formats(url: &str) -> Result<YtDlpResponse, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    let output = Command::new("yt-dlp")
        .args([
            "--dump-json",
            "--no-download",
            "--no-playlist",
            url,
        ])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}. Make sure yt-dlp is installed and available in PATH.", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp failed: {}", error_msg));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let response: YtDlpResponse = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse yt-dlp response: {}", e))?;

    Ok(response)
}

#[tauri::command]
async fn download_video(url: &str, format_id: &str, output_path: &str) -> Result<String, String> {
    let output = Command::new("yt-dlp")
        .args(["-f", format_id, "-o", output_path, url])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Download failed: {}", error_msg));
    }
    Ok("Download completed successfully".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            fetch_formats,
            download_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

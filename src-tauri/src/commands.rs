use super::{models::YtDlpResponse, utils::get_download_dir};
use std::process::Command;

#[tauri::command]
pub async fn fetch_formats(url: &str) -> Result<YtDlpResponse, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    let output = Command::new("yt-dlp")
        .args(["--dump-json", "--no-download", "--no-playlist", url])
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute yt-dlp: {}. Make sure yt-dlp is installed and available in PATH.",
                e
            )
        })?;

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
pub async fn download(
    url: String,
    video_format: Option<String>,
    audio_format: Option<String>,
    default_format: Option<String>,
) -> Result<String, String> {
    let format_selection = if let Some(format_id) = default_format {
        format_id
    } else {
        match (video_format, audio_format) {
            (Some(vid), Some(aud)) => format!("{}+{}", vid, aud),
            (Some(vid), None) => vid,
            (None, Some(aud)) => aud,
            (None, None) => return Err("No format selected".to_string()),
        }
    };

    let downloads_dir =
        get_download_dir().ok_or_else(|| "Could not determine downloads directory".to_string())?;
    let var_name = r#"%(title)s.%(ext)s"#;
    let output_template = downloads_dir.join(var_name).to_string_lossy().to_string();

    let output = Command::new("yt-dlp")
        .args(["-f", &format_selection, "-o", &output_template, &url])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Download failed: {}", error_msg));
    }

    let success_msg = "Download completed successfully".to_string();

    Ok(success_msg)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct YtDlpFormat {
    pub format_id: String,
    pub format_note: Option<String>,
    pub ext: String,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub filesize: Option<i64>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub fps: Option<f64>,
    pub tbr: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YtDlpResponse {
    pub id: String,
    pub title: String,
    pub formats: Vec<YtDlpFormat>,
}

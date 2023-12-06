use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Thumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Serialize, Deserialize)]
pub struct Wallhaven {
    id: String,
    url: String,
    short_url: String,
    views: u32,
    favorites: u32,
    source: String,
    purity: String,
    category: String,
    dimension_x: u32,
    dimension_y: u32,
    resolution: String,
    ratio: String,
    file_size: u32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    pub path: String,
    thumbs: Thumbs,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub data: Vec<Wallhaven>,
}

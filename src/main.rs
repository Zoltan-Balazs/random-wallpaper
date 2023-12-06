pub mod cli;
pub mod wallhaven;

use clap::Parser;
use cli::Args;
use log::{info, LevelFilter};
use rand::Rng;
use systemd_journal_logger::JournalLog;
use wallhaven::Response;

// https://wallhaven.cc/help/api
const BASE_URL: &str = "https://wallhaven.cc/api/v1/search?";
const API_KEY: &str = ""; // Only needed for NSFW content
const RATIOS: &str = "16x9"; // 16x9, 16x10
const SORTING: &str = "toplist"; // date_added, relevance, random, views, favorites, toplist
const ORDER: &str = "desc"; // desc, asc
const TOPRANGE: &str = "1y"; // 1d, 3d, 1w, 1M, 3M, 6M, 1y - ONLY for 'toplist' sorting

fn main() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(LevelFilter::Info);

    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let url = format!(
        "{}apikey={}&categories={}&purity={}&ratios={}&sorting={}&order={}&topRange={}&page={}",
        BASE_URL,
        API_KEY,
        args.categories,
        args.purity,
        RATIOS,
        SORTING,
        ORDER,
        TOPRANGE,
        args.page
    );

    info!("Fetching background: {}", url);
    let response = reqwest::blocking::get(&url).expect(&format!("No response on url {}", url));
    let json: Response = response.json().expect("Cannot parse json");

    let index = rng.gen_range(0..json.data.len());
    let chosen = json
        .data
        .get(index)
        .expect(&format!("Cannot find index {} in json", index));

    info!("Attempting to change background to: {}", &chosen.path);
    wallpaper::set_from_url(&chosen.path).expect("Unable to set wallpaper");
}

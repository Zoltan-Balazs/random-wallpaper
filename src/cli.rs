use clap::Parser;
use clap::{arg, command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// What categories to search for, 1 - on, 0 - off; (General, Anime, People)
    #[arg(short, long, default_value_t = 100)]
    pub categories: u8,

    /// What purity to search for,  1 - on, 0 - off; (SFW, Sketchy, NSFW)
    #[arg(short, long, default_value_t = 100)]
    pub purity: u8,

    /// Which page to get the wallpaper from
    #[arg(short, long, default_value_t = 1)]
    pub page: u8,
}

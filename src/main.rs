use ansi_term::Colour::Fixed;
use chrono;
use clap::App;
use clap::ArgMatches;
use std::fs::create_dir;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::Path;
mod types;

// CRATE-PACKAGE VARIABLES
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
// COLORS
const PURPLE: u8 = 177;
const CYAN: u8 = 159;
const BRONZE: u8 = 208;
// TODO: choices for download dir should be more interactive
const DOWNLOAD_DIR_LOCATION: &str = "img";

fn is_valid_wallpaper_link(s: &str) -> bool {
    if s.ends_with(".jpg") || s.ends_with(".png") {
        return true;
    }
    false
}

async fn download_link(url: &str, file_name: &str, dir_name: &str) -> types::Res<()> {
    if dir_name.ends_with("/") || dir_name.ends_with("\\") {
        dir_name.to_string().pop();
    }

    let mut file_content = Cursor::new(reqwest::get(url).await?.bytes().await?);
    let mut output_file = File::create(format!("{}/{}", dir_name, file_name))
        .expect("Failed to create file for downloaded image");

    copy(&mut file_content, &mut output_file).expect("Failed to copy content");

    Ok(())
}

#[tokio::main]
async fn fetch_from_reddit(args: ArgMatches) -> types::Res<()> {
    // TODO: when making the script more interactive, user will be able to choose /r themselves
    let reddit_url = format!(
        "https://www.reddit.com/r/{}/top.json?t=all&limit=100",
        "wallpaper"
    );
    let res = reqwest::get(&reddit_url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let value: types::RedditResponse = serde_json::from_str(&body)?;
    // TODO: can be used to give users info at a later point
    let mut link_number: i32 = 0;

    for wallpaper in value.data.children {
        if is_valid_wallpaper_link(&wallpaper.data.url) {
            link_number += 1;
            let response = reqwest::get(&wallpaper.data.url).await?;

            let file_name_with_author = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| {
                    if name.is_empty() {
                        None
                    } else {
                        Some(format!("{}_{}", wallpaper.data.author, name))
                    }
                })
                .unwrap_or("tmp.bin".to_string());

            if !Path::new(DOWNLOAD_DIR_LOCATION).exists() {
                create_dir(DOWNLOAD_DIR_LOCATION).expect("failed to create dir");
            }

            download_link(
                &wallpaper.data.url,
                file_name_with_author.as_str(),
                DOWNLOAD_DIR_LOCATION,
            )
            .await?;

            let time = chrono::Local::now().format("%T");

            println!(
                "{} | Downloading: {} | Upvotes: {} | Sub: r/{}",
                Fixed(PURPLE).paint(format!("[{}]", &time.to_string())),
                Fixed(CYAN).paint(&file_name_with_author.to_string()),
                Fixed(BRONZE).paint(&wallpaper.data.ups.to_string()),
                &wallpaper.data.subreddit,
            );
        }
    }

    println!("after = {}", value.data.after);

    Ok(())
}

fn main() {
    let args = App::new("rusty_wallpaper_scraper")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .get_matches();

    fetch_from_reddit(args).expect("failed");
}

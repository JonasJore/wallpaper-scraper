use clap::App;
use clap::ArgMatches;
use chrono;
use std::fs::create_dir;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::Path;
use ansi_term::Colour::Fixed;

mod types;

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

    // let file_name_with_author = ;

    let mut output_file = File::create(format!("{}/{}", dir_name, file_name))
        .expect("Failed to create file for downloaded image");

    copy(&mut file_content, &mut output_file).expect("Failed to copy content");

    Ok(())
}

#[tokio::main]
async fn fetch_from_reddit(args: ArgMatches) -> types::Res<()> {
    let reddit_url = format!("https://www.reddit.com/r/{}/top.json?t=all&limit=100", "wallpaper");
    let res = reqwest::get(&reddit_url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let value: types::RedditResponse = serde_json::from_str(&body)?;
    let mut link_number: i32 = 0; // TODO: use this when this program becomes more interactive

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

            if !Path::new("img").exists() {
                create_dir("img").expect("failed to create dir");
            }

            download_link(&wallpaper.data.url, file_name_with_author.as_str(),"img").await?;

            let time = chrono::Local::now().format("%T");

            println!(
                "{} | Downloading: {} | Upvotes: {} | Sub: r/{}",
                Fixed(177).paint(format!("[{}]", &time.to_string())),
                Fixed(159).paint(&file_name_with_author.to_string()),
                Fixed(208).paint(&wallpaper.data.ups.to_string()),
                &wallpaper.data.subreddit,
            );
        }
    }

    println!("after = {}", value.data.after);

    Ok(())
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESC: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let args = App::new("rusty_wallpaper_scraper")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESC)
        .get_matches();

    fetch_from_reddit(args).expect("failed")
}

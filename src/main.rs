use std::{fs::File, env::temp_dir};
use std::io;
use std::io::copy;
use std::path::Path;
use tempfile::Builder;

mod types;

fn is_valid_wallpaper_link(s: &str) -> bool {
    if s.ends_with(".jpg") || s.ends_with(".png") {
        return true;
    }

    false
}

#[tokio::main]
async fn fetch_from_reddit() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.reddit.com/r/wallpaper/top.json?t=all&limit=100")
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let value: types::RedditResponse = serde_json::from_str(&body)?;
    let mut link_number: i32 = 0;

    for wallpaper in value.data.children {
        if is_valid_wallpaper_link(&wallpaper.data.url) {
            println!("{}", wallpaper.data.url);
            link_number += 1;

            let tmp_dir = Builder::new().prefix("downloaded").tempdir()?;
            let response = reqwest::get(&wallpaper.data.url).await?;

            let mut destination = {
                let file_name = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                println!("file to download: {}", file_name);
                let file_name = tmp_dir.path().join(file_name);
                File::create(file_name)?
            };

            let co = response.text().await?;

            copy(&mut co.as_bytes(), &mut destination);
        }
        println!("{}", link_number);
    }

    println!("after = {}", value.data.after);

    Ok(())
} 

fn main() {
    fetch_from_reddit();
}
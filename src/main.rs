use std::fs::File;
use std::io::copy;

mod types;

fn is_valid_wallpaper_link(s: &str) -> bool {
    if s.ends_with(".jpg") || s.ends_with(".png") {
        return true;
    }

    return false;
}

async fn download_link(url: &str, file_name: &str, dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if dir_name.ends_with("/") || dir_name.ends_with("\\") {
        dir_name.to_string().pop();
    }
    let mut resp = reqwest::get(url).await?;
    let mut out = File::create(format!("{}/{}", dir_name, file_name)).expect("failed to create file");
    copy(&mut resp.text().await?.as_bytes(), &mut out).expect("failed to copy content");
    Ok(())
}

#[tokio::main]
async fn fetch_from_reddit() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.reddit.com/r/wallpaper/top.json?t=all&limit=100").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let value: types::RedditResponse = serde_json::from_str(&body)?;
    let mut link_number: i32 = 0;

    for wallpaper in value.data.children {
        if is_valid_wallpaper_link(&wallpaper.data.url) {
            println!("{}", wallpaper.data.url);
            link_number += 1;

            let response = reqwest::get(&wallpaper.data.url).await?;

            let file_name = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            println!("file to download: {}", file_name);

            download_link(&wallpaper.data.url, file_name, "img").await?;
        }
        println!("{}", link_number);
    }

    println!("after = {}", value.data.after);

    Ok(())
}

fn main() {
    fetch_from_reddit();
}

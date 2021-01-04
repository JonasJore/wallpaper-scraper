use std::{ fs::File };
use std::io;

mod types;

fn is_valid_wallpaper_link(s: &str) -> bool {
    if s.ends_with(".jpg") || s.ends_with(".png") {
        return true;
    }

    return false;
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
            println!("link: {}", wallpaper.data.url);
            link_number += 1;

            let mut resp = reqwest::get(&wallpaper.data.url);
            let mut out = File::create(format!("{}/{}", "dir_name", &link_number.to_string())).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy content");
        }
        println!("{}", link_number);
    }

    println!("after = {}", value.data.after);

    Ok(())
} 

fn main() {
    fetch_from_reddit();
}
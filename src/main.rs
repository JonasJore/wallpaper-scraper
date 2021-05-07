use std::fs::create_dir;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::Path;

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

    let mut output_file =
        File::create(format!("{}/{}", dir_name, file_name)).expect("failed to create file");
    copy(&mut file_content, &mut output_file).expect("failed to copy content");

    Ok(())
}

#[tokio::main]
async fn fetch_from_reddit() -> types::Res<()> {
    let res = reqwest::get("https://www.reddit.com/r/wallpaper/top.json?t=all&limit=100").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let value: types::RedditResponse = serde_json::from_str(&body)?;
    let mut link_number: i32 = 0;

    for wallpaper in value.data.children {
        if is_valid_wallpaper_link(&wallpaper.data.url) {
            link_number += 1;
            let response = reqwest::get(&wallpaper.data.url).await?;

            let file_name = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            

            if !Path::new("img").exists() {
                create_dir("img").expect("failed to create dir");
            }

            download_link(&wallpaper.data.url, file_name, "img").await?;
            
            println!("Downloading: {} | Upvotes: {} | By: {} | Sub: r/{} | Downloaded: {} images", 
                &file_name,
                &wallpaper.data.ups,
                &wallpaper.data.author,
                &wallpaper.data.subreddit,
                &link_number
            );
        }
    }

    println!("after = {}", value.data.after);

    Ok(())
}

fn main() {
    fetch_from_reddit().expect("failed");
}

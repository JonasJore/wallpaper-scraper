use serde::Deserialize;

#[derive(Deserialize)]
pub struct WallpaperPost {
    subreddit: String,
    ups: u32,
    pub url: String,
    created: f64,
    author: String,
}
#[derive(Deserialize)]
pub struct Post {
    pub data: WallpaperPost
}

#[derive(Deserialize)]
pub struct Data {
    pub children: Vec<Post>,
    pub after: String, // TODO: support for pagination later
}

#[derive(Deserialize)]
pub struct RedditResponse {
    pub data: Data,
}
use serde::Deserialize;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize)]
pub struct WallpaperPost {
    pub subreddit: String,
    pub ups: u32,
    pub url: String,
   //created: f64, TODO: implement use for this later
    pub author: String,
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
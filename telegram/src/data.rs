#[derive(Debug, Default)]
pub struct Channel {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub posts: Vec<ChannelPost>,
}

impl Channel {
    pub fn preview_url(&self) -> String {
        format!("https://t.me/s/{}", self.slug)
    }
    pub fn url(&self) -> String {
        format!("https://t.me/{}", self.slug)
    }
}

#[derive(Debug, Default)]
pub struct ChannelPost {
    /// channel_slug/id
    pub id: String,
    pub text: String,
    pub html: String,
    pub datetime: String,

    pub media: Option<Vec<Media>>,
    pub file: Option<Vec<File>>,
    pub forwarded_from: Option<ForwardedFrom>,
    pub link_preview: Option<LinkPreview>,
    pub poll: Option<Poll>,
    pub from_author: Option<String>,
}

impl ChannelPost {
    pub fn preview_url(&self) -> String {
        format!("https://t.me/s/{}", self.id)
    }
    pub fn url(&self) -> String {
        format!("https://t.me/{}", self.id)
    }
}

#[derive(Debug)]
pub enum Media {
    Photo(String),
    Video { url: String, thumb_url: String },
    VideoGif { url: String, thumb_url: String },
    VideoTooBig { thumb_url: String },
}

#[derive(Debug, Default)]
pub struct File {
    pub filename: String,
    pub size: String,
}

#[derive(Debug, Default)]
pub struct LinkPreview {
    pub url: String,
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub media: Option<Media>,
}

#[derive(Debug, Default)]
pub struct ForwardedFrom {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct Poll {
    pub question: String,
    pub r#type: String,
    pub options: Vec<PollOption>
}

#[derive(Debug, Default)]
pub struct PollOption {
    pub name: String,
    pub percent: String,
}
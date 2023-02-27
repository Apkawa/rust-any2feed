use std::collections::HashMap;
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::utils::format_url;

#[derive(Debug, Deserialize)]
pub struct MeweApiIdentify {
    pub authenticated: bool,
    pub confirmed: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiSelfProfileInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub contact_invite_id: String,
    pub timezone: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiUserInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub contact_invite_id: String,
    pub name: String,
}


#[derive(Debug, Deserialize)]
pub struct MeweApiFeedList {
    pub feed: Vec<MeweApiPost>,
    pub users: Vec<MeweApiUserInfo>,
    #[serde(rename = "_links")]
    pub links: Option<MeweApiFeedListNextPageLink>,

    pub groups: Option<Vec<MeweApiGroup>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiFeedListNextPageLink {
    pub next_page: Option<MeweApiHref>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiPost {
    #[serde(rename = "postItemId")]
    pub id: String,
    pub user_id: String,
    pub text: String,

    // Хоть тут и написано что utc, на самом деле приходит с таймзоной клиента
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,

    // #[serde(with = "ts_seconds_option")]
    // pub edited_at: Option<DateTime<Utc>>,
    pub edited_at: Option<usize>,

    pub group_id: Option<String>,
    // Media
    pub medias: Option<Vec<MeweApiMedia>>,
    pub medias_count: Option<usize>,
    pub photos_count: Option<usize>,
    // Link
    pub link: Option<MeweApiLink>,
    // Ref post
    pub ref_post: Option<Box<MeweApiPost>>,
    // Poll
    pub poll: Option<MeweApiPoll>,
    // Files
    pub files: Option<Vec<MeweApiMediaFile>>,
    // Sticker
    // TODO
    pub stickers: Option<Vec<MeweApiSticker>>,

    pub album: Option<String>,
    pub hash_tags: Option<Vec<String>>,
}

impl MeweApiPost {
    pub fn get_post_url(&self, user: Option<&MeweApiUserInfo>) -> Option<String> {
        let user_id = &self.user_id;
        match self.group_id.as_ref() {
            Some(group_id) => return Some(format!("https://mewe.com/group/{group_id}/profile/{user_id}")),
            None => {
                if let Some(MeweApiUserInfo { contact_invite_id, .. }) = user {
                    return Some(format!("https://mewe.com/i/{contact_invite_id}"));
                }
            }
        }
        None
    }
}

// COMMON

#[derive(Debug, Deserialize)]
pub struct MeweApiHref {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct MeweApiMediaSize {
    pub width: usize,
    pub height: usize,
}

// MEDIA

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiMedia {
    #[serde(rename = "mediaId")]
    pub id: String,
    #[serde(rename = "postItemId")]
    pub post_id: String,
    pub photo: MeweApiMediaPhoto,
    pub video: Option<MeweApiMediaVideo>,
}

// Media Photo

#[derive(Debug, Deserialize)]
pub struct MeweApiMediaPhoto {
    pub id: String,
    pub size: MeweApiMediaSize,
    pub mime: String,

    #[serde(rename = "_links")]
    pub links: MeweApiMediaPhotoLink,
}

impl MeweApiMediaPhoto {
    pub fn url(&self) -> String {
        let url = &self.links.img.href;
        let args: HashMap<&str, &str> = HashMap::from(
            [("imageSize", "200x300"), ("static", "0")]);
        let url = format_url(url.as_str(), &args);
        let mime = &self.mime;
        format!("https://mewe.com{url}&mime={mime}")
    }
}


#[derive(Debug, Deserialize)]
pub struct MeweApiMediaPhotoLink {
    pub img: MeweApiHref,
}

// MediaVideo

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiMediaVideo {
    pub id: String,
    pub name: String,
    pub available_resolutions: Vec<String>,

    #[serde(rename = "_links")]
    pub links: MeweApiMediaVideoLink,
}

impl MeweApiMediaVideo {
    pub fn url(&self) -> String {
        let url = &self.links.link_template.href;
        let args: HashMap<&str, &str> = HashMap::from(
            [("resolution", "original")]);
        let url = format_url(url.as_str(), &args);
        let name = &self.name;
        format!("https://mewe.com{url}&mime=video/mp4&name={name}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiMediaVideoLink {
    pub link_template: MeweApiHref,
}
// Media File

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiMediaFile {
    pub id: String,
    pub file_name: String,
    pub mime: String,
    pub length: usize,
    pub file_type: String,
    #[serde(rename = "_links")]
    pub links: MeweApiMediaFileLinks,
}

#[derive(Debug, Deserialize)]
pub struct MeweApiMediaFileLinks {
    pub url: MeweApiHref,
}

// LINK

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiLink {
    pub title: String,
    pub description: String,
    pub thumbnail_size: Option<MeweApiMediaSize>,
    #[serde(rename = "_links")]
    pub links: MeweApiLinkLinks,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiLinkLinks {
    pub url: MeweApiHref,
    pub url_host: MeweApiHref,
    pub thumbnail: Option<MeweApiHref>,
}
// Poll

#[derive(Debug, Deserialize)]
pub struct MeweApiPollOptionPhoto {
    pub id: String,
    pub size: MeweApiMediaSize,

    #[serde(rename = "_links")]
    pub links: MeweApiMediaPhotoLink,
}

#[derive(Debug, Deserialize)]
pub struct MeweApiPollOption {
    pub text: String,
    pub image: Option<MeweApiPollOptionPhoto>,
    pub votes: usize,
    pub selected: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MeweApiPoll {
    pub question: String,
    pub options: Vec<MeweApiPollOption>,
}

// Sticker
#[derive(Debug, Deserialize)]
pub struct MeweApiSticker {
    pub id: String,
    /// for get sticker see https://cdn.mewe.com/emoji/build-info.json
    /// As example:
    /// For { "package": "stickers-summer-fun_free", "id": "lit"}
    /// https://cdn.mewe.com/emoji/stickers-summer-fun_free/lit.1sum9.svg
    /// https://cdn.mewe.com/emoji/stickers-summer-fun_free/lit.1sum9.png
    ///
    /// We need get `1sum9` from build-info for construct url. maybe hardcoded.
    pub package: String,
}

// Groups

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiGroupList {
    pub confirmed_groups: Vec<MeweApiGroup>,
    pub unconfirmed_groups: Vec<MeweApiGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "descriptionPlain")]
    pub description: Option<String>,
    pub is_muted: Option<bool>,
}

// Contacts

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiContactList {
    pub contacts: Vec<MeweApiContact>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiContact {
    pub id: String,
    pub user: MeweApiContactUser,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiContactUser {
    pub close_friend: Option<bool>,
    pub id: String,
    pub contact_invite_id: String,
    pub name: String,
}




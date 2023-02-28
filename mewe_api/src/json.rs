use std::collections::HashMap;
use chrono::serde::{ts_seconds};
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

impl MeweApiUserInfo {
    pub fn url(&self, group_id: Option<&String>) -> String {
        let user_id = &self.id;

        match group_id {
            Some(group_id) => format!("https://mewe.com/group/{group_id}/profile/{user_id}"),
            None => {
                format!("https://mewe.com/i/{}", self.contact_invite_id)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MeweApiFeedList {
    pub feed: Vec<MeweApiPost>,
    pub users: Vec<MeweApiUserInfo>,
    #[serde(rename = "_links")]
    pub links: Option<MeweApiFeedListNextPageLink>,

    pub groups: Option<Vec<MeweApiGroup>>,
}

impl MeweApiFeedList {
    pub fn next_page(&self) -> Option<String> {
        if let Some(MeweApiFeedListNextPageLink { next_page: Some(page) }) = &self.links {
            Some(["https://mewe.com/", page.href.as_str()].join(""))
        } else {
            None
        }
    }
    pub fn fill_user_and_group(&mut self) {
        // todo into serde
        let mut users: HashMap<&String, &MeweApiUserInfo> = self.users.iter().map(|u| (&u.id, u)).collect();
        let mut groups: HashMap<&String, &MeweApiGroup> = HashMap::with_capacity(20);
        for user in self.users.iter() {
            users.insert(&user.id, user);
        }
        if let Some(list_groups) = self.groups.as_ref() {
            for group in list_groups.iter() {
                groups.insert(&group.id, group);
            }
        }
        for post in &mut self.feed {
            let user = users.get(&post.user_id).copied();
            let group = post.group_id.as_ref()
                .and_then(|id| groups.get(&id)).copied();
            post.user = user.map(|u| (*u).clone());
            post.group = group.map(|g| (*g).clone());
            if post.ref_post.is_some() {
                let mut ref_post = post.ref_post.as_mut().unwrap();
                let user = users.get(&ref_post.user_id).copied();
                let group = ref_post.group_id.as_ref()
                    .and_then(|id| groups.get(&id)).copied();
                ref_post.user = user.map(|u| (*u).clone());
                ref_post.group = group.map(|u| (*u).clone());
            }
        }
    }
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
    #[serde(skip)]
    pub user: Option<MeweApiUserInfo>,
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
    #[serde(skip)]
    pub group: Option<MeweApiGroup>,

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
    pub fn url(&self) -> Option<String> {
        self.user.as_ref().map(|u| u.url(self.group_id.as_ref()))
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
            [
                ("imageSize", "800x800"), // 400x400, 800x800, 1600x1600
                ("static", "0")
            ]);
        let url = format_url(url.as_str(), &args);
        format!("https://mewe.com{url}")
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "descriptionPlain")]
    pub description: Option<String>,
    pub is_muted: Option<bool>,
}

impl MeweApiGroup {
    pub fn url(&self) -> String {
        format!("https://mewe.com/group/{}", self.id)
    }
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




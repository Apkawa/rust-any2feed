use serde::Deserialize;

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
}

#[derive(Debug, Deserialize, Clone)]
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
    #[serde(rename="_links")]
    pub links: Option<MeweApiFeedListNextPageLink>,

    #[serde(skip)]
    pub groups: Option<()>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiFeedListNextPageLink {
    pub next_page: Option<MeweApiHref>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiPost {
    #[serde(rename = "postItemId")]
    pub id: String,
    pub user_id: String,
    pub text: String,
    pub created_at: usize,
    pub updated_at: usize,
    pub group_id: Option<String>,
    // Media
    pub medias: Option<Vec<MeweApiMedia>>,
    pub medias_count: Option<usize>,
    pub photos_count: Option<usize>,
    // Link
    pub link: Option<MeweApiLink>,
    // Ref post
    pub ref_post: Option<Box<MeweApiPost>>,
    // Pool
    // Files
    //


    pub album: Option<String>,
    pub hash_tags: Option<Vec<String>>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeweApiMediaVideoLink {
    pub link_template: MeweApiHref,
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
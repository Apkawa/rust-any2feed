pub struct DanbooruImage {
    hash: String,
    ext: String,
    // w150: String,
    // w180: String,
    // w225: String,
    // w270: String,
    // w360: String,
    // w720: String,
}

const IMAGE_URL: &str = "https://cdn.donmai.us";

impl DanbooruImage {
    pub fn from_md5(hash: &str, ext: &str) -> DanbooruImage {
        DanbooruImage {
            hash: hash.to_string(),
            ext: ext.to_string(),
        }
    }
    pub fn prefix(&self) -> String {
        let hash = &self.hash;
        let s1 = &hash[0..2];
        let s2 = &hash[2..4];
        format!("{s1}/{s2}/{hash}")
    }
    pub fn build_url(&self, variant: &str, ext: Option<&str>) -> String {
        let prefix = self.prefix();
        let ext = ext.unwrap_or("jpg");
        format!("{IMAGE_URL}/{variant}/{prefix}.{ext}")
    }
    pub fn original(&self) -> String {
        self.build_url("original", Some(&self.ext))
    }
    pub fn sample(&self) -> String {
        self.build_url("sample", None)
    }
    pub fn preview(&self) -> String {
        self.build_url("preview", None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let i = DanbooruImage::from_md5("3c5073a2fa2c3d4041a25d75fdbb27f8", "png");
        assert_eq!(
            i.original(),
            "https://cdn.donmai.us/original/3c/50/3c5073a2fa2c3d4041a25d75fdbb27f8.png".to_string()
        );
        assert_eq!(
            i.sample(),
            "https://cdn.donmai.us/sample/3c/50/3c5073a2fa2c3d4041a25d75fdbb27f8.jpg".to_string()
        );
        assert_eq!(
            i.preview(),
            "https://cdn.donmai.us/preview/3c/50/3c5073a2fa2c3d4041a25d75fdbb27f8.jpg".to_string()
        );
    }
}

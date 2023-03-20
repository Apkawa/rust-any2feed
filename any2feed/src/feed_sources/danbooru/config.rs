use std::collections::HashMap;
use std::fmt::format;
use reqwest::Url;
use serde::Deserialize;
use booru_rs::manager::Engine;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) danbooru: DanbooruFeedSourceConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DanbooruFeedSourceConfig {
    pub(crate) proxy: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) limit: Option<u32>,
}

impl DanbooruFeedSourceConfig {
    pub(crate) fn limit(&self) -> u32 {
        self.limit.unwrap_or(50)
    }
}

#[derive(Debug)]
pub(crate) struct BooruConfig {
    sites: HashMap<String, BooruSiteConfig>,
}

impl BooruConfig {
    /// Load normalized_config
    pub fn load(toml_str: &str) -> BooruConfig {
        let config = toml::from_str::<ConfigTOML>(toml_str).unwrap().booru;
        let sites_capacity = config.site.len() + config.engines.iter().map(|(_, c)| c.len()).sum::<usize>();
        let mut sites: HashMap<String, BooruSiteConfig> = HashMap::with_capacity(sites_capacity);
        let engines = config.engines.into_iter()
            .chain(config.site.into_iter()
                .map(|s| (s.engine.clone(), s))
                .filter(|(e, _)| e.is_some()) // TODO detection engine from url
                .map(|(e, s)|(e.unwrap().to_owned(), vec![s]))
            );
        for (engine, engine_sites) in engines {
            for s in engine_sites {
                let limit = s.limit.unwrap_or(config.limit.unwrap_or(50));
                let mut tags = HashMap::with_capacity(s.tags.len());
                for t in s.tags.iter() {
                    let tag = match t {
                        BooruTagEnum::Tag(tag) => BooruTag {
                            tag: tag.to_owned(),
                            limit,
                            order: s.order.to_owned(),
                            rating: s.rating.to_owned(),
                        },
                        BooruTagEnum::TagConfig {
                            tag, order,
                            rating,
                            limit: l
                        } => {
                            BooruTag {
                                tag: tag.to_owned(),
                                limit: l.unwrap_or(limit),
                                order: order.as_ref().or_else(|| s.order.as_ref()).cloned(),
                                rating: rating.as_ref().or_else(|| s.rating.as_ref()).cloned(),
                            }
                        }
                    };
                    tags.insert(tag.tag.clone(), tag);
                }
                let proxy = if let Some(proxy) = s.proxy.as_ref() {
                    match proxy {
                        BooruProxyEnum::ProxyDisabled(flag) => {
                            if *flag {
                                config.proxy.as_ref()
                            } else {
                                None
                            }
                        }
                        BooruProxyEnum::ProxyOverride(proxy) => Some(proxy)
                    }
                } else {
                    config.proxy.as_ref()
                };
                let site_config = BooruSiteConfig {
                        engine: engine.to_owned(),
                        url: s.url.to_owned(),
                        proxy: proxy.cloned(),
                        tags,
                    };
                let mut key = site_config.engine.to_string();
                if let Some(url) = site_config.url.as_ref() {
                    let url = Url::parse(url).unwrap();
                    let host = url.host_str().unwrap();
                    key.push_str(format!("-{host}").as_str())
                }
                sites.insert(key, site_config);
            }
        }

        sites.shrink_to_fit();
        BooruConfig {
            sites
        }
    }
}

#[derive(Debug)]
pub(crate) struct BooruSiteConfig {
    engine: Engine,
    url: Option<String>,
    proxy: Option<String>,
    tags: HashMap<String, BooruTag>,
}

#[derive(Debug)]
pub(crate) struct BooruTag {
    tag: String,
    limit: u32,
    order: Option<String>,
    rating: Option<String>,
}

// Serde
#[derive(Debug, Deserialize)]
pub(crate) struct ConfigTOML {
    booru: GlobalBooruConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GlobalBooruConfig {
    limit: Option<u32>,
    proxy: Option<String>,
    #[serde(default = "Vec::new")]
    site: Vec<SiteConfig>,
    #[serde(flatten)]
    engines: HashMap<Engine, Vec<SiteConfig>>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct SiteConfig {
    engine: Option<Engine>,
    url: Option<String>,
    limit: Option<u32>,
    proxy: Option<BooruProxyEnum>,
    order: Option<String>,
    rating: Option<String>,
    tags: Vec<BooruTagEnum>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum BooruProxyEnum {
    ProxyDisabled(bool),
    ProxyOverride(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum BooruTagEnum {
    Tag(String),
    TagConfig {
        tag: String,
        order: Option<String>,
        rating: Option<String>,
        limit: Option<u32>,
    },
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_minimal() {
        let toml = r#"
[booru]
# Global limit
limit = 10
# Global proxy
proxy = "https://host:port"
        "#;

        let config = BooruConfig::load(toml);
        dbg!(&config);
    }

    #[test]
    fn config_sites() {
        let toml = r#"
[booru]
# Global limit
limit = 10
# Global proxy
proxy = "https://host:port"

[[booru.site]]
# optional, try autodetection engine from host
engine = "danbooru"
# optional if engine set, use default engine host
url = "https://testbooru.donmai.us"
# Disable proxy
proxy = false
limit = 5
# Optional
order = "id"
# Optioal
rating = "s"

tags = [
    "", # no tags, all posts
    "foo bar",
    "1girl",
    { tag = "1girl", rating = "s", order = "id", limit = 100 }
]
        "#;

        let config = BooruConfig::load(toml);
        dbg!(&config);
    }

    #[test]
    fn config() {
        let toml = r#"
[booru]
# Global limit
limit = 10
# Global proxy
proxy = "https://host:port"

#
[[booru.site]]
# optional, try autodetection engine from host
engine = "danbooru"
# optional if engine set, use default engine host
url = "https://testbooru.donmai.us"
# Disable proxy
proxy = false
limit = 5
# Optional
order = "id"
# Optioal
rating = "s"

tags = [
    "", # no tags, all posts
    "foo bar",
    "1girl",
    { tag = "1girl", rating = "s", order = "id", limit = 100 }
]

[[booru.danbooru]]
# optional
url = "https://testbooru.donmai.us"

limit = 5
tags = [
    "foo bar",
    "1girl"
]

[[booru.gelbooru_v02]]
# optional
url = "https://testbooru.donmai.us"

limit = 5
tags = [
    "foo bar",
    "1girl"
]
        "#;

        let config = BooruConfig::load(toml);
        dbg!(&config);
    }
}
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) channels: HashMap<String, ExtraChannelConfig>,
    // Default
    pub(crate) pages: Option<usize>,
}

impl Config {
    /// Load normalized_config
    pub fn load(toml_str: &str) -> Config {
        let config_toml = toml::from_str::<ConfigTOML>(toml_str).unwrap().telegram;
        // Нормализация настроек
        let mut channels: HashMap<String, ExtraChannelConfig> = HashMap::with_capacity(
            // Вычисляем заранее размер таблицы
            config_toml.channels.as_ref().map(|c| c.len()).unwrap_or(0)
                + config_toml
                    .extra
                    .as_ref()
                    .map(|e| e.channel_map.len())
                    .unwrap_or(0),
        );

        use self::ChannelConfig::*;
        if let Some(channels_vec) = config_toml.channels {
            // Собираем словарь  из [telegram].channels = [..]
            for ch in channels_vec {
                let mut config = ExtraChannelConfig::default();
                let slug = match ch {
                    Slug(x) => x,
                    WithOptions { slug, pages } => {
                        config.pages = pages;
                        slug
                    }
                };
                channels.entry(slug).or_insert(config).pages = config.pages
            }
        }
        // Добираем словарь из [telegram.extra.channel_name]
        if let Some(ExtraChannelMap { channel_map }) = config_toml.extra {
            for (slug, extra) in channel_map {
                let mut config = channels
                    .entry(slug)
                    .or_insert_with(ExtraChannelConfig::default);
                config.pages = extra.pages;
            }
        }

        Config {
            channels,
            pages: config_toml.pages,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigTOML {
    telegram: TelegramConfig,
}

#[derive(Debug, Deserialize)]
struct TelegramConfig {
    channels: Option<Vec<ChannelConfig>>,
    pages: Option<usize>,
    extra: Option<ExtraChannelMap>,
}

#[derive(Debug, Deserialize)]
struct ExtraChannelMap {
    #[serde(flatten)]
    channel_map: HashMap<String, ExtraChannelConfig>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub(crate) struct ExtraChannelConfig {
    pub(crate) pages: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ChannelConfig {
    Slug(String),
    WithOptions { slug: String, pages: Option<usize> },
}

#[cfg(test)]
mod test {
    use crate::feed_sources::telegram::config::Config;
    use std::fs::read_to_string;
    use test_utils::fixture::path_from_git_root;

    #[test]
    fn test_config_empty() {
        let config = Config::load("[telegram]");
        dbg!(&config);
    }

    #[test]
    fn test_config_example() {
        let config_path = path_from_git_root("./any2feed_config_example.toml").unwrap();
        let config_str = read_to_string(config_path).unwrap();
        let config = Config::load(config_str.as_str());
        dbg!(&config);
    }

    #[test]
    fn test_config() {
        let config_str = r#"
        [telegram]
        # List of channels useful for opml export and override o
        channels = [
            "oper_goblin",
            "dvachannel",
            # With config
            { slug = "foo_channel", pages = 2 },
            # Only slug
            { slug = "foo_channel" }
        ]
        # For initial sync channel all records
        pages = 1

        # Add and/or override per channel config
        [telegram.extra.channel_name]
        # Num page for specific channel
        pages = 5

        # Maybe empty
        [telegram.extra.channel_name_2]
        [telegram.extra.channel_name_3]
        "#;

        let config = Config::load(config_str);
        dbg!(&config);
        assert_eq!(config.pages, Some(1));
        // TODO compare 2 hashmap
        // assert_eq!(config.channels, HashMap::from([
        //     ("foo_channel", ExtraChannelConfig::default()),
        //     ("channel_name_2", ExtraChannelConfig::default()),
        //     ("channel_name", ExtraChannelConfig { pages: Some(5) }),
        //     ("oper_goblin", ExtraChannelConfig::default()),
        //     ("dvachannel", ExtraChannelConfig::default()),
        // ].map(|(k, v)| (k.to_string(), v)))
        // )
    }
}

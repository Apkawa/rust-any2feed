use crate::importers::traits::RenderContent;
use telegram::data::{ChannelPost, File, LinkPreview, Media, Poll};

impl RenderContent for ChannelPost {
    fn render(&self) -> Option<String> {
        let mut content = String::with_capacity(self.html.len() * 2);

        if let Some(f) = self.forwarded_from.as_ref() {
            content
                .push_str(format!(r#"<a href="{}">Forwarded from {}</a>"#, f.url, f.name).as_str());
        }
        let parts = [
            Some(format!("<p>{}</p>", &self.html)),
            self.media.render(),
            self.link_preview.render(),
            self.poll.render(),
        ]
        .into_iter()
        .flatten()
        .collect::<String>();

        content.push_str(parts.as_str());
        content.shrink_to_fit();
        Some(content)
    }
}

impl RenderContent for Media {
    fn render(&self) -> Option<String> {
        match &self {
            Media::Photo(url) => Some(format!(r#"<img src="{url}" />"#)),
            Media::Voice(url) => Some(format!(r#"<audio controls src="{url}"></audio>"#)),
            Media::Video { url, thumb_url } | Media::VideoGif { url, thumb_url } => {
                let attrs = if let Media::VideoGif { .. } = &self {
                    "autoplay muted loop playsinline".to_string()
                } else {
                    "controls".to_string()
                };
                Some(format!(
                    r#"
                <video style="max-width: 800px; height: auto" poster="{thumb_url}" {attrs}>
                   <source src="{url}" type="video/mp4" />
                   <object data="{url}" />
                </video>
                "#
                ))
            }
            Media::VideoTooBig { thumb_url } => {
                // TODO прокинуть урл вида t.me/channel/id чтобы перейти в телегу на просмотр видео
                Some(format!(
                    r#"
                <p><i>MEDIA TOO BIG</i></p>
                <img src="{thumb_url}" />
                "#
                ))
            }
        }
    }
}

impl RenderContent for LinkPreview {
    fn render(&self) -> Option<String> {
        let thumbnail = {
            if let Some(t) = self.media.as_ref() {
                t.render().unwrap()
            } else {
                String::new()
            }
        };
        let content = format!(
            r#"
        <blockquote>
          <p style="white-space:pre-wrap;"><b>{title}</b></p>
          <p style="white-space:pre-wrap;">
          {site_name}: <a href="{url}" style="white-space:pre-wrap;">{url}</a>
          </p>
          {thumbnail}
          <p style="white-space:pre-wrap;">{description}</p>
        </blockquote>"#,
            thumbnail = thumbnail,
            title = &self.title,
            url = &self.url,
            description = &self.description,
            site_name = &self.site_name,
        );
        Some(content)
    }
}

impl RenderContent for File {
    fn render(&self) -> Option<String> {
        todo!()
    }
}

impl RenderContent for Poll {
    fn render(&self) -> Option<String> {
        let options: String = self
            .options
            .iter()
            .map(|o| format!("<li>{} - {}</li>\n", o.name, o.percent))
            .collect();
        Some(format!(
            r#"
        <p>
            <span>{t}: {question}</span>
            <ul>
                {options}
            </ul>

        </p>
        "#,
            t = self.r#type,
            question = self.question,
            options = options
        ))
    }
}

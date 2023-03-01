use telegram::data::{ChannelPost, File, LinkPreview, Media, Poll};
use crate::importers::traits::RenderContent;

impl RenderContent for ChannelPost {
    fn render(&self) -> Option<String> {
        let mut content = String::with_capacity(self.html.len() * 2);

        if let Some(f) = self.forwarded_from.as_ref() {
            content.push_str(format!(r#"<a href="{}">Forwarded from {}</a>"#, f.url, f.name).as_str());
        }
        let parts = [
            Some(format!("<p>{}</p>", &self.html)),
            self.media.render(),
            self.link_preview.render(),
            self.poll.render(),
        ].into_iter()
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect::<String>();

        content.push_str(parts.as_str());
        content.shrink_to_fit();
        Some(content)
    }
}

impl RenderContent for Media {
    fn render(&self) -> Option<String> {
        match self {
            Media::Photo(url) => Some(format!(r#"<img src="{url}" />"#)),
            Media::Video { url, thumb_url } | Media::VideoGif { url, thumb_url } => {
                Some(format!(r#"
                <video poster="{thumb_url}" controls>
                   <source src="{url}" type="video/mp4">
                   <object data="{url}" width="470" height="255">
                </video>
                "#))
            }
            Media::VideoTooBig { thumb_url } => {
                Some(format!(r#"<img src="{thumb_url}" />"#))
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
        let content = format!(r#"
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
        let options: String = self.options
            .iter()
            .map(|o| format!("<li>{} - {}</li>\n", o.name, o.percent))
            .collect();
        Some(format!(r#"
        <p>
            <span>{t}: {question}</span>
            <ul>
                {options}
            </ul>

        </p>
        "#, t = self.r#type, question = self.question, options = options))
    }
}

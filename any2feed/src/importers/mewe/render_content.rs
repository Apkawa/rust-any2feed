use std::collections::HashMap;
use crate::importers::mewe::json::{MeweApiLink, MeweApiMedia, MeweApiMediaPhoto, MeweApiMediaVideo, MeweApiPost};
use crate::importers::mewe::markdown::md_to_html;
use crate::importers::mewe::utils::format_url;


pub struct RenderContext {}

pub trait RenderContent {
    fn as_dyn(&self) -> &dyn RenderContent where Self: Sized {
        self
    }
    fn render(&self) -> Option<String>;
}


impl RenderContent for MeweApiPost {
    fn render(&self) -> Option<String> {
        let mut content = md_to_html(&self.text);
        let parts: Vec<Option<Box<&dyn RenderContent>>> = vec![
            self.link.as_ref().map(|l| Box::new(l.as_dyn())),
        ];
        if self.ref_post.is_some() {
            if let Some(r) = self.ref_post.as_ref().unwrap().render() { content.push_str(r.as_str()) }
        }
        if self.medias.is_some() {
            for m in self.medias.as_ref().unwrap() {
                if let Some(r) = m.render() { content.push_str(r.as_str()) }
            }
        }

        let parts = parts.iter()
            .filter(|i| i.is_some())
            .filter_map(|i| i.as_ref().unwrap().render())
            .collect::<String>()
            ;

        content.push_str(parts.as_str());
        Some(content)
    }
}


impl RenderContent for MeweApiLink {
    fn render(&self) -> Option<String> {
        let thumbnail = {
            if let Some(t) = self.links.thumbnail.as_ref() {
                format!(r#" <img src="{href}"></img> "#, href = t.href)
            } else {
                String::new()
            }
        };
        let content = format!(r#"
        <blockquote>
          <p style="white-space:pre-wrap;"><b>{title}</b></p>
          <p style="white-space:pre-wrap;">
          URL: <a href="{url}" style="white-space:pre-wrap;">{url}</a>
          </p>
          {thumbnail}
          <p style="white-space:pre-wrap;">{description}</p>
        </blockquote>"#,
                              thumbnail = thumbnail,
                              title = &self.title, url = &self.links.url.href,
                              description = &self.description
        );
        Some(content)
    }
}

impl RenderContent for MeweApiMedia {
    fn render(&self) -> Option<String> {
        let url = &self.photo.render_url();
        match self.video.as_ref() {
            Some(video) => {
                let video_url = &video.render_url();
                // let text = photo.links.img
                let width = usize::min(self.photo.size.width, 640);
                Some(format!(r#"
            <video width="{width}" height="auto" controls=1
                poster="{url}"\>
            <source src="{video_url}" type="video/mp4" />
            </video>
                "#))
            }
            None => {
                Some(format!(r#"<img src="{url}" />"#))
            }
        }
    }
}


impl MeweApiMediaPhoto {
    fn render_url(&self) -> String {
        let url = &self.links.img.href;
        let args: HashMap<&str, &str> = HashMap::from(
            [("imageSize", "200x300"), ("static", "0")]);
        let url = format_url(url.as_str(), &args);
        let mime = &self.mime;
        format!("https://mewe.com{url}&mime={mime}")
    }
}

impl MeweApiMediaVideo {
    fn render_url(&self) -> String {
        let url = &self.links.link_template.href;
        let args: HashMap<&str, &str> = HashMap::from(
            [("resolution", "original")]);
        let url = format_url(url.as_str(), &args);
        let name = &self.name;
        format!("https://mewe.com{url}&mime=video/mp4&name={name}")
    }
}

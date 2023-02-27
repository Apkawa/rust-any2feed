
use mewe_api::json::{MeweApiLink, MeweApiMedia, MeweApiMediaFile, MeweApiPoll, MeweApiPost};
use mewe_api::markdown::md_to_html;



pub struct RenderContext {}

pub trait RenderContent {
    fn as_dyn(&self) -> &dyn RenderContent where Self: Sized {
        self
    }
    fn render(&self) -> Option<String>;
}

// TODO initialize
// static RE_GIFYCAT: regex::Regex = regex::Regex::new(r#"(https://thumbs.gfycat.com/[^<\s]+)"#).unwrap();

///
/// ```
/// use any2feed::importers::mewe::render_content::render_gifycat_gifs;
/// let res = render_gifycat_gifs("Foo https://thumbs.gfycat.com/a.gif#h=1200w=1600");
/// assert_eq!(res, r#"Foo <p><img src="https://thumbs.gfycat.com/a.gif#h=1200w=1600" /></p>"#.to_string());
/// let res = render_gifycat_gifs("<p>Foo https://thumbs.gfycat.com/a.gif#h=1200w=1600</p>");
/// assert_eq!(res, r#"<p>Foo <p><img src="https://thumbs.gfycat.com/a.gif#h=1200w=1600" /></p></p>"#.to_string());
/// ```
pub fn render_gifycat_gifs(text: &str) -> String {
    let re = regex::Regex::new(r#"(https://thumbs.gfycat.com/[^<\s]+)\b"#).unwrap();
    re.replace(text, r#"<p><img src="$1" /></p>"#).to_string()
}

impl RenderContent for MeweApiPost {
    fn render(&self) -> Option<String> {
        let mut content = md_to_html(&self.text);
        content = render_gifycat_gifs(content.as_str());

        let parts: Vec<Option<Box<&dyn RenderContent>>> = vec![
            self.link.as_ref().map(|l| Box::new(l.as_dyn())),
            self.poll.as_ref().map(|l| Box::new(l.as_dyn())),
        ];
        // reshared post
        if let Some(ref_post) = self.ref_post.as_ref() {
            if let Some(ref_user) = ref_post.user.as_ref() {
                content.push_str("<p>");
                content.push_str(format!(r#"<a href="{}">{}</a>"#,
                                     ref_post.url().unwrap(),
                                     ref_user.name).as_str());
                if let Some(group) = ref_post.group.as_ref() {
                    content.push_str(format!(r#" - <a href="{}">{}</a>"#,
                                         group.url(),
                                         group.name).as_str())
                };
                content.push_str("</p>");
            }
            if let Some(r) = ref_post.render() { content.push_str(r.as_str()) }
        }

        // Medias
        if self.medias.is_some() {
            if let Some(album) = self.album.as_ref() {
                content.push_str(format!("<p>Album: <b>{album}</b></p>").as_str());
            }
            for m in self.medias.as_ref().unwrap() {
                if let Some(r) = m.render() {
                    content.push_str(r.as_str())
                }
            }
        }
        // Files
        if self.files.is_some() {
            for m in self.files.as_ref().unwrap() {
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
        let url = &self.photo.url();
        match self.video.as_ref() {
            Some(video) => {
                let video_url = &video.url();
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




// File

impl RenderContent for MeweApiMediaFile {
    fn render(&self) -> Option<String> {
        let name = &self.file_name;
        let url = &self.links.url.href;
        let size = &((self.length as f64) / 1024.0 / 1024.0);

        Some(format!(r#"<p>File: <a href="https://mewe.com{url}">{name} ({size:.2} MB)</a></p>"#))
    }
}


// Poll
impl RenderContent for MeweApiPoll {
    fn render(&self) -> Option<String> {
        let poll_options: String = self.options
            .iter()
            // TODO percent
            .map(|o| format!("<li>{} - {}</li>\n", o.text, o.votes))
            .collect();
        let question = &self.question;
        Some(format!(r#"
        <p>
            Question: {question}
            <ul>
                {poll_options}
            </ul>
        </p>
        "#))
    }
}

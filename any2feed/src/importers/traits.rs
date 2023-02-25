use feed::Feed;

pub trait Importer {
    fn import(&self) -> Feed;
}

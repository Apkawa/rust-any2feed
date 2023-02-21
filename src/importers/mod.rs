use crate::feed::Feed;

mod traits;


trait Importer {
    fn import(&self) -> Feed;
}
use crate::importers::mewe::importer::MeweImporter;
use crate::importers::telegram::TelegramImporter;
use crate::importers::traits::Importer;

pub mod mewe;
pub mod telegram;
pub mod traits;

pub struct ImporterList;

impl ImporterList {
    pub fn get_importers(toml: &str) -> Vec<Box<dyn Importer>> {
        vec![
            Box::new(MeweImporter::with_config(toml)),
            Box::new(TelegramImporter::with_config(toml)),
        ]
    }
}

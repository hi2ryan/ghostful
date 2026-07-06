use crate::{config::BrowserConfig, error::Result, page::Page};

pub struct Browser {}

impl Browser {
    pub async fn launch(config: BrowserConfig) -> Result<Self> {
        todo!()
    }

    pub async fn new_page(&self, url: impl AsRef<str>) -> Result<Page> {
        let url = url.as_ref();

        todo!()
    }

    pub async fn close(&self) -> Result<()> {
        todo!()
    }
}

use std::sync::Arc;

use crate::{element::Element, error::Result, frame::Frame, input::InputController};

pub struct Page {
    input: Arc<InputController>,
}

impl Page {
	pub fn input(&self) -> Arc<InputController> {
		Arc::clone(&self.input)
	}

    pub async fn goto(&self, url: impl AsRef<str>) -> Result<()> {
        let url = url.as_ref();
        todo!()
    }

    pub async fn url(&self) -> String {
        todo!()
    }

    pub async fn reload(&self) -> Result<()> {
        todo!()
    }

	pub async fn wait_for(&self, selector: impl AsRef<str>) -> Result<Element> {
		let selector = selector.as_ref();

		todo!()
	}

	pub async fn wait_for_visible(&self, selector: impl AsRef<str>) -> Result<Element> {
		let selector = selector.as_ref();

		todo!()
	}

    pub async fn main_frame(&self) -> Result<Frame> {
        todo!()
    }

    pub async fn frames(&self) -> Result<Vec<Frame>> {
        todo!()
    }

    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        todo!()
    }
}

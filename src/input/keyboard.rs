use crate::{
    error::Result,
    input::{InputController, Point},
};

impl InputController {
    pub fn type_text(&self, point: Point, text: impl AsRef<str>) -> Result<()> {
        todo!()
    }

    pub fn human_type(&self, point: Point, text: impl AsRef<str>) -> Result<()> {
        todo!()
    }
}

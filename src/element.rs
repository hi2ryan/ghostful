use crate::{error::Result, input::Point, page::Page};

pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn center(&self) -> Point {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

pub struct Element<'a> {
    page: &'a Page,
    node_id: i32,
}

impl<'a> Element<'a> {
    pub async fn bounding_box(&self) -> Result<BoundingBox> {
        todo!()
    }

    pub async fn center(&self) -> Result<Point> {
        self.bounding_box().await.map(|bounds| bounds.center())
    }

    pub async fn click(&self) -> Result<()> {
        let center = self.center().await?;
        self.page.input().click(center)
    }

    pub async fn human_click(&self) -> Result<()> {
        let center = self.center().await?;
        self.page.input().human_click(center)
    }

    pub async fn type_text(&self, text: impl AsRef<str>) -> Result<()> {
        let text = text.as_ref();
        let center = self.center().await?;
        self.page.input().type_text(center, text)
    }

    pub async fn human_type(&self, text: impl AsRef<str>) -> Result<()> {
        let text = text.as_ref();
        let center = self.center().await?;
        self.page.input().human_type(center, text)
    }
}

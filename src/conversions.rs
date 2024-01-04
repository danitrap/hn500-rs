use crate::models::HnItem;
pub trait ConvertibleToHnItem {
    fn convert_to_hn_item(&self) -> Option<HnItem>;
}

impl ConvertibleToHnItem for rss::Item {
    fn convert_to_hn_item(&self) -> Option<HnItem> {
        let title = self.title()?;
        let snippet = self.description()?;
        let guid = self.guid()?.value();
        Some(HnItem::new(title, snippet, guid))
    }
}

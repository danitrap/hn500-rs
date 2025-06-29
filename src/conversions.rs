//! This module contains the logic for converting between different data models.

use crate::models::HnItem;

/// A trait for converting an object to an `HnItem`.
pub trait ConvertibleToHnItem {
    /// Converts the object to an `HnItem`.
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_converts_rss_item_to_hn_item() {
        let mut rss_item = rss::Item::default();
        let mut guid = rss::Guid::default();
        rss_item.set_title("title".to_string());
        rss_item.set_description("snippet".to_string());
        guid.set_value("00000000-0000-0000-0000-00000000000");
        rss_item.set_guid(guid);

        let hn_item = rss_item.convert_to_hn_item().unwrap();
        assert_eq!(hn_item.title, "title");
        assert_eq!(hn_item.snippet, "snippet");
        assert_eq!(hn_item.guid, "00000000-0000-0000-0000-00000000000");
    }

    #[test]
    fn it_fails_to_convert_rss_item_to_hn_item_when_something_is_missing() {
        let mut rss_item = rss::Item::default();
        let mut guid = rss::Guid::default();
        rss_item.set_title("title".to_string());
        guid.set_value("00000000-0000-0000-0000-00000000000");
        rss_item.set_guid(guid);
        let hn_item = rss_item.convert_to_hn_item();
        assert_eq!(hn_item, None);
    }
}

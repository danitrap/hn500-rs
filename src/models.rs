use crate::utils::strip_html;
use std::collections::{HashSet, LinkedList};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct HnItem {
    pub title: String,
    pub snippet: String,
}

impl HnItem {
    pub fn new(title: String, snippet: String) -> Self {
        let snippet = strip_html(snippet);

        Self { title, snippet }
    }
}

impl fmt::Display for HnItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\n{}", self.title, self.snippet)
    }
}

enum AddItemResult<'a> {
    Added(&'a HnItem),
    AlreadyExists,
}

pub struct HackerNews {
    items: HashSet<HnItem>,
    history: LinkedList<HnItem>,
}

impl HackerNews {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            history: LinkedList::new(),
        }
    }

    pub fn whats_new<'a>(&mut self, items: &'a Vec<HnItem>) -> Vec<&'a HnItem> {
        let mut new_items = Vec::new();

        for item in items {
            if let AddItemResult::Added(new_item) = self.add_item(item) {
                new_items.push(new_item);
            }
        }

        self.truncate();

        new_items
    }

    fn add_item<'a>(&mut self, item: &'a HnItem) -> AddItemResult<'a> {
        if self.items.contains(item) {
            return AddItemResult::AlreadyExists;
        }

        self.items.insert(item.clone());
        self.history.push_front(item.clone());

        AddItemResult::Added(item)
    }

    fn truncate(&mut self) {
        let len = self.history.len();

        if len <= 100 {
            return;
        }

        let excess = len - 100;

        for _ in 0..excess {
            if let Some(item) = self.history.pop_back() {
                self.items.remove(&item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_instance_of_hnitem() {
        let instance = HnItem::new("title".to_string(), "snippet".to_string());
        assert_eq!(instance.title, "title");
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_strips_html_from_snippet() {
        let instance = HnItem::new("title".to_string(), "<p>snippet</p>".to_string());
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_adds_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string());
        instance.add_item(&item);
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_does_not_add_duplicate_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string());
        instance.add_item(&item);
        instance.add_item(&item);
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_truncates_history() {
        let mut instance = HackerNews::new();
        let mut items = Vec::new();
        assert_eq!(instance.history.len(), 0);

        for i in 0..111 {
            let unique_title = format!("title{}", i);
            let item = HnItem::new(unique_title, "snippet".to_string());
            items.push(item);
        }

        instance.whats_new(&items);

        assert_eq!(instance.history.len(), 100);
    }

    #[test]
    fn it_retains_the_latest_items_when_it_truncates() {
        let mut instance = HackerNews::new();
        let mut items = Vec::new();

        assert_eq!(instance.history.len(), 0);

        for i in 0..111 {
            let unique_title = format!("title{}", i);
            let item = HnItem::new(unique_title, "snippet".to_string());
            items.push(item);
        }

        instance.whats_new(&items);

        assert_eq!(instance.history.len(), 100);
        assert_eq!(instance.history.front().unwrap().title, "title110");
        assert_eq!(instance.history.back().unwrap().title, "title11");
    }

    #[test]
    fn it_returns_new_items() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string());
        let items = vec![item.clone()];
        let new_items = instance.whats_new(&items);

        assert_eq!(new_items.len(), 1);
        assert_eq!(new_items[0], &item);
    }

    #[test]
    fn it_does_not_return_items_that_are_already_in_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string());
        instance.add_item(&item);
        let items = vec![item.clone()];
        let new_items = instance.whats_new(&items);

        assert_eq!(new_items.len(), 0);

        let item2 = HnItem::new("title2".to_string(), "snippet2".to_string());
        let items2 = vec![item.clone(), item2.clone()];
        let new_items2 = instance.whats_new(&items2);

        assert_eq!(new_items2.len(), 1);
        assert_eq!(new_items2[0], &item2);
    }
}

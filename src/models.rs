#![deny(clippy::all)]

use crate::utils::strip_html;
use std::collections::{HashSet, LinkedList};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct HnItem {
    pub title: String,
    pub snippet: String,
    pub guid: String,
}

impl HnItem {
    pub fn new(title: String, snippet: String, guid: String) -> Self {
        let snippet = strip_html(snippet);

        Self {
            title,
            snippet,
            guid,
        }
    }
}

impl Hash for HnItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.guid.hash(state);
    }
}

impl PartialEq for HnItem {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
    }
}

impl Eq for HnItem {}

impl fmt::Display for HnItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\n{}", self.title, self.snippet)
    }
}

enum AddItemResult {
    Added(Rc<HnItem>),
    AlreadyExists,
}

pub struct HackerNews {
    items: HashSet<Rc<HnItem>>,
    history: LinkedList<Rc<HnItem>>,
}

impl HackerNews {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            history: LinkedList::new(),
        }
    }

    pub fn whats_new(&mut self, items: Vec<HnItem>) -> Option<Vec<Rc<HnItem>>> {
        let new_items = items
            .into_iter()
            .filter_map(|item| match self.add_item_if_not_exists(item) {
                AddItemResult::Added(new_item) => Some(new_item),
                AddItemResult::AlreadyExists => None,
            })
            .collect::<Vec<_>>();

        self.truncate();

        if new_items.is_empty() {
            None
        } else {
            Some(new_items)
        }
    }

    fn add_item_if_not_exists(&mut self, item: HnItem) -> AddItemResult {
        match self.items.contains(&item) {
            true => AddItemResult::AlreadyExists,
            false => {
                let item = Rc::new(item);
                self.items.insert(Rc::clone(&item));
                self.history.push_front(Rc::clone(&item));
                AddItemResult::Added(item)
            }
        }
    }

    fn truncate(&mut self) {
        while self.history.len() > 100 {
            match self.history.pop_back() {
                Some(item) => self.items.remove(&item),
                None => break,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_instance_of_hnitem() {
        let instance = HnItem::new(
            "title".to_string(),
            "snippet".to_string(),
            "guid".to_string(),
        );
        assert_eq!(instance.title, "title");
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_strips_html_from_snippet() {
        let instance = HnItem::new(
            "title".to_string(),
            "<p>snippet</p>".to_string(),
            "".to_string(),
        );
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_adds_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string(), "".to_string());
        instance.add_item_if_not_exists(item);
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_does_not_add_duplicate_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new(
            "title".to_string(),
            "snippet".to_string(),
            "guid".to_string(),
        );
        instance.add_item_if_not_exists(item.clone());
        instance.add_item_if_not_exists(item);
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_truncates_history() {
        let mut instance = HackerNews::new();
        let mut items = Vec::new();
        assert_eq!(instance.history.len(), 0);

        for i in 0..111 {
            let item = HnItem::new("title".to_string(), "snippet".to_string(), i.to_string());
            items.push(item);
        }

        instance.whats_new(items);

        assert_eq!(instance.history.len(), 100);
    }

    #[test]
    fn it_retains_the_latest_items_when_it_truncates() {
        let mut instance = HackerNews::new();
        let mut items = Vec::new();

        assert_eq!(instance.history.len(), 0);

        for i in 0..111 {
            let numbered_title = format!("title{}", i);
            let item = HnItem::new(numbered_title, "snippet".to_string(), i.to_string());
            items.push(item);
        }

        instance.whats_new(items);

        assert_eq!(instance.history.len(), 100);
        assert_eq!(instance.history.front().unwrap().title, "title110");
        assert_eq!(instance.history.back().unwrap().title, "title11");
    }

    #[test]
    fn it_returns_new_items() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title".to_string(), "snippet".to_string(), "".to_string());
        let items = vec![item.clone()];
        let new_items = instance.whats_new(items);

        assert_eq!(new_items, Some(vec![Rc::new(item)]));
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_does_not_return_items_that_are_already_in_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new(
            "title".to_string(),
            "snippet".to_string(),
            "guid".to_string(),
        );
        instance.add_item_if_not_exists(item.clone());
        let items = vec![item.clone()];
        let new_items = instance.whats_new(items);

        assert_eq!(new_items, None);

        let item2 = HnItem::new(
            "title2".to_string(),
            "snippet2".to_string(),
            "guid 2".to_string(),
        );
        let items2 = vec![item.clone(), item2.clone()];
        let new_items2 = instance.whats_new(items2);

        assert_eq!(new_items2, Some(vec![Rc::new(item2)]));
    }
}

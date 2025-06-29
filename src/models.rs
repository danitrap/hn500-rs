//! This module contains the data models for the application.

#![deny(clippy::all)]

use crate::utils::strip_html;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Represents the possible errors that can occur in the application.
pub enum ApplicationError {
    /// An error occurred while fetching the Hacker News feed.
    Fetching,
    /// An error occurred while parsing the Hacker News feed.
    Parsing,
    /// The first run of the application is being skipped.
    SkippingFirstRun,
    /// There are no new items in the Hacker News feed.
    NoNewItems,
}

/// Represents a Hacker News item.
#[derive(Clone, Debug)]
pub struct HnItem {
    /// The title of the item.
    pub title: String,
    /// The snippet of the item.
    pub snippet: String,
    /// The GUID of the item.
    pub guid: String,
}

impl HnItem {
    /// Creates a new `HnItem` instance.
    pub fn new(title: &str, snippet: &str, guid: &str) -> Self {
        let snippet = strip_html(snippet);

        Self {
            title: title.into(),
            snippet,
            guid: guid.into(),
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

/// Represents the Hacker News feed.
pub struct HackerNews {
    items: HashSet<Rc<HnItem>>,
    history: VecDeque<Rc<HnItem>>,
}

impl HackerNews {
    /// Creates a new `HackerNews` instance.
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            history: VecDeque::new(),
        }
    }

    /// Returns the new items in the feed.
    pub fn whats_new(&mut self, items: Vec<HnItem>) -> Option<Vec<Rc<HnItem>>> {
        let new_items = items
            .into_iter()
            .filter_map(|item| match self.add_item_if_not_exists(item) {
                AddItemResult::Added(new_item) => Some(new_item),
                AddItemResult::AlreadyExists => None,
            })
            .collect::<Vec<_>>();

        self.truncate();
        log::debug!("History now contains {} items", self.history.len());

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
        let instance = HnItem::new("title", "snippet", "guid");
        assert_eq!(instance.title, "title");
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_strips_html_from_snippet() {
        let instance = HnItem::new("title", "<p>snippet</p>", "");
        assert_eq!(instance.snippet, "snippet");
    }

    #[test]
    fn it_adds_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title", "snippet", "");
        instance.add_item_if_not_exists(item);
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_does_not_add_duplicate_items_to_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title", "snippet", "guid");
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
            let item = HnItem::new("title", "snippet", &i.to_string());
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
            let item = HnItem::new(&numbered_title, "snippet", &i.to_string());
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
        let item = HnItem::new("title", "snippet", "");
        let items = vec![item.clone()];
        let new_items = instance.whats_new(items);

        assert_eq!(new_items, Some(vec![Rc::new(item)]));
        assert_eq!(instance.history.len(), 1);
    }

    #[test]
    fn it_does_not_return_items_that_are_already_in_history() {
        let mut instance = HackerNews::new();
        let item = HnItem::new("title", "snippet", "guid");
        instance.add_item_if_not_exists(item.clone());
        let items = vec![item.clone()];
        let new_items = instance.whats_new(items);

        assert_eq!(new_items, None);

        let item2 = HnItem::new("title2", "snippet2", "guid 2");
        let items2 = vec![item.clone(), item2.clone()];
        let new_items2 = instance.whats_new(items2);

        assert_eq!(new_items2, Some(vec![Rc::new(item2)]));
    }
}

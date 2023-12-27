use crate::utils::strip_html;
use std::collections::{HashSet, LinkedList};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq)]
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

pub struct HackerNews {
    items: HashSet<HnItem>,
    previous_items: HashSet<HnItem>,
    history: LinkedList<HnItem>,
}

impl HackerNews {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            previous_items: HashSet::new(),
            history: LinkedList::new(),
        }
    }

    pub fn add_item(&mut self, item: HnItem) {
        if self.items.contains(&item) {
            return;
        }

        self.items.insert(item.clone());
        self.history.push_front(item);

        self.truncate();
    }

    pub fn get_new_items(&mut self) -> Vec<HnItem> {
        let mut new_items = Vec::new();

        for item in self.items.difference(&self.previous_items) {
            new_items.push(item.clone());
        }

        self.previous_items = self.items.clone();

        new_items
    }

    fn truncate(&mut self) {
        if self.history.len() > 100 {
            if let Some(last_item) = self.history.pop_back() {
                self.items.remove(&last_item);
            }
        }
    }
}

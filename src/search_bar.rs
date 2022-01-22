use std::fmt::{self, Display};

pub struct SearchBar {
    prefix: String,
    query: Vec<char>,
    // Position in the query
    cursor: usize,
}

impl SearchBar {
    pub fn new() -> Self {
        SearchBar {
            prefix: "[jqi]> ".to_string(),
            query: Vec::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, c: char) {
        if self.cursor <= self.query.len() {
            self.query.insert(self.cursor, c);
            self.cursor += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 && self.cursor <= self.query.len() {
            self.query.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.query.len() {
            self.query.remove(self.cursor);
        }
        if self.cursor >= self.query.len() {
            self.move_cursor_left();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor < self.query.len() {
            self.cursor += 1;
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor + self.prefix.len()
    }

    pub fn query(&self) -> String {
        self.query.iter().collect()
    }
}

impl Display for SearchBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.query())
    }
}

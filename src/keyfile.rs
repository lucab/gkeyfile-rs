//! Keyfile type.

use crate::Value;
use crate::{errors, parser, writer};
use std::collections::HashMap;

/// Groups of key-value pairs.
#[derive(Debug, Default)]
pub struct Keyfile {
    pub(crate) groups: HashMap<String, HashMap<String, Value>>,
}

impl Keyfile {
    /// Return a new empty keyfile.
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    /// Add a key-value pair into a given group.
    pub fn insert(&mut self, group: String, key: String, value: Value) {
        let pairs = self.groups.entry(group).or_default();
        pairs.insert(key, value);
    }

    /// Return an iterator for group labels.
    pub fn groups_labels(&self) -> impl Iterator<Item = &str> {
        self.groups.keys().map(|s| s.as_ref())
    }

    /// Parse a keyfile from a buffered reader.
    pub(crate) fn parse_buf<T>(bufrd: T) -> errors::Result<Self>
    where
        T: std::io::BufRead,
    {
        parser::parse_buf(bufrd)
    }

    /// Write a keyfile to a buffered writer.
    pub(crate) fn write_buf<T>(&self, bufwr: &mut std::io::BufWriter<T>) -> errors::Result<()>
    where
        T: std::io::Write,
    {
        writer::write_buf(self, bufwr)
    }
}

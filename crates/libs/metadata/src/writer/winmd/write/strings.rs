use super::Write;
use std::collections::hash_map::*;

pub struct Strings<'a> {
    map: HashMap<&'a str, u32>,
    stream: Vec<u8>,
}

impl<'a> Default for Strings<'a> {
    fn default() -> Self {
        Self { map: Default::default(), stream: vec![0] }
    }
}

impl<'a> Strings<'a> {
    pub fn insert(&mut self, value: &'a str) -> u32 {
        if value.is_empty() {
            return 0;
        }

        match self.map.entry(value) {
            Entry::Vacant(entry) => {
                let offset = *entry.insert(self.stream.len() as _);
                self.stream.extend_from_slice(value.as_bytes());
                self.stream.push(0);
                offset
            }
            Entry::Occupied(entry) => *entry.get(),
        }
    }

    pub fn into_stream(self) -> Vec<u8> {
        self.stream.into_stream()
    }
}

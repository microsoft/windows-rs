use super::*;
use std::collections::hash_map::*;

pub struct Strings {
    map: HashMap<String, u32>,
    stream: Vec<u8>,
}

impl Default for Strings {
    fn default() -> Self {
        Self { map: Default::default(), stream: vec![0] }
    }
}

impl Strings {
    pub fn insert(&mut self, value: &str) -> u32 {
        if value.is_empty() {
            return 0;
        }

        match self.map.entry(value.to_string()) {
            Entry::Vacant(entry) => {
                let offset = *entry.insert(self.stream.len() as u32);
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

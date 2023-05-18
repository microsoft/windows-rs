use super::Write;
use std::collections::hash_map::*;

pub struct Blobs {
    map: HashMap<Vec<u8>, u32>,
    stream: Vec<u8>,
}

impl Default for Blobs {
    fn default() -> Self {
        Self { map: Default::default(), stream: vec![0] }
    }
}

impl Blobs {
    pub fn insert(&mut self, value: &[u8]) -> u32 {
        if value.is_empty() {
            return 0;
        }

        match self.map.entry(value.to_vec()) {
            Entry::Vacant(entry) => {
                let offset = *entry.insert(self.stream.len() as _);
                let len = value.len();
                match len {
                    0..=0x7F => self.stream.push(len as _),
                    0x80..=0x3FFF => {
                        self.stream.push((0x80 | len >> 8) as _);
                        self.stream.push((0xFF & len) as _);
                    }
                    _ => {
                        self.stream.push((0xC0 | len >> 24) as _);
                        self.stream.push((0xFF & len >> 16) as _);
                        self.stream.push((0xFF & len >> 8) as _);
                        self.stream.push((0xFF & len) as _);
                    }
                }
                self.stream.extend_from_slice(value);
                offset
            }
            Entry::Occupied(entry) => *entry.get(),
        }
    }

    pub fn into_stream(self) -> Vec<u8> {
        self.stream.into_stream()
    }
}

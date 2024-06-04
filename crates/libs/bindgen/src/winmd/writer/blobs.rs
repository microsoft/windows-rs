use super::*;

pub struct Blobs {
    map: HashMap<Vec<u8>, u32>,
    stream: Vec<u8>,
}

impl Default for Blobs {
    fn default() -> Self {
        Self {
            map: Default::default(),
            stream: vec![0],
        }
    }
}

impl Blobs {
    pub fn insert(&mut self, value: &[u8]) -> u32 {
        if value.is_empty() {
            return 0;
        }

        match self.map.entry(value.to_vec()) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                let offset = *entry.insert(self.stream.len() as u32);
                let len = value.len();
                match len {
                    0..=0x7F => self.stream.push(len as u8),
                    0x80..=0x3FFF => {
                        self.stream.push((0x80 | len >> 8) as u8);
                        self.stream.push((0xFF & len) as u8);
                    }
                    _ => {
                        self.stream.push((0xC0 | len >> 24) as u8);
                        self.stream.push((0xFF & len >> 16) as u8);
                        self.stream.push((0xFF & len >> 8) as u8);
                        self.stream.push((0xFF & len) as u8);
                    }
                }
                self.stream.extend_from_slice(value);
                offset
            }
            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
        }
    }

    pub fn into_stream(self) -> Vec<u8> {
        self.stream.into_stream()
    }
}

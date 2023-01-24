use super::*;

#[derive(Default)]
pub struct Blobs {
    // Blobs don't need to be sorted. A map is used to collapse duplicates. A `BTreeMap` in particular is used to help with reproducible builds.
    map: BTreeMap<Vec<u8>, u32>,
    stream: Vec<u8>,
}

pub struct StagedBlobs(Blobs);

impl Blobs {
    pub fn insert(&mut self, value: Vec<u8>) {
        if !value.is_empty() {
            self.map.entry(value).or_default();
        }
    }

    pub fn stage(mut self) -> StagedBlobs {
        self.stream = vec![0];

        for (value, index) in self.map.iter_mut() {
            *index = self.stream.len() as _;

            match value.len() {
                0..=0x7F => self.stream.push(value.len() as _),
                0x80..=0x3FFF => {
                    self.stream.push((0x80 | value.len() >> 8) as _);
                    self.stream.push((0xFF & value.len()) as _);
                }
                _ => {
                    self.stream.push((0xC0 | value.len() >> 24) as _);
                    self.stream.push((0xFF & value.len() >> 16) as _);
                    self.stream.push((0xFF & value.len() >> 8) as _);
                    self.stream.push((0xFF & value.len()) as _);
                }
            }

            self.stream.extend_from_slice(value);
        }

        self.stream.resize(round(self.stream.len(), 4), 0);
        StagedBlobs(self)
    }
}

impl StagedBlobs {
    pub fn stream(self) -> Vec<u8> {
        self.0.stream
    }

    #[track_caller]
    pub fn index(&self, value: &[u8]) -> u32 {
        if value.is_empty() {
            0
        } else {
            self.0.map[value]
        }
    }
}

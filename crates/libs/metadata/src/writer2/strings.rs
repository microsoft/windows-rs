use super::*;

#[derive(Default)]
pub struct Strings<'a> {
    // Strings don't need to be sorted. A map is used to collapse duplicates. A `BTreeMap` in particular is used to help with reproducible builds.
    map: BTreeMap<&'a str, u32>,
    stream: Vec<u8>,
}

impl<'a> Strings<'a> {
    pub fn insert(&mut self, value: &'a str) {
        if !value.is_empty() {
            self.map.entry(value).or_default();
        }
    }

    pub fn stage(&mut self) -> &[u8] {
        self.stream = vec![0];

        for (value, index) in self.map.iter_mut() {
            *index = self.stream.len() as _;

            self.stream.extend_from_slice(value.as_bytes());
            self.stream.push(0); // terminator
        }

        self.stream.resize(round(self.stream.len(), 4), 0);
        &self.stream
    }

    pub fn get(&self, value: &str) -> u32 {
        if value.is_empty() {
            0
        } else {
            self.map[value]
        }
    }
}

use crate::*;

pub struct Strings {
    set: BTreeMap<String, usize>,
    stream: Vec<u8>,
}

impl Strings {
    pub fn new() -> Self {
        Self { set: BTreeMap::new(), stream: vec![0] }
    }

    pub fn insert(&mut self, value: &str) -> u32 {
        let pos = self.stream.len();
        let mut insert = false;

        let pos = *self.set.entry(value.to_string()).or_insert_with(|| {
            insert = true;
            pos
        });

        if insert {
            self.stream.extend_from_slice(value.as_bytes());
            self.stream.push(0); // terminator
        }

        pos as _
    }

    pub fn into_stream(mut self) -> Vec<u8> {
        self.stream.resize(round(self.stream.len(), 4), 0);
        self.stream
    }
}

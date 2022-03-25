use super::*;

pub(crate) struct Blobs {
    set: BTreeMap<Vec<u8>, usize>,
    stream: Vec<u8>,
}

impl Blobs {
    pub fn new() -> Self {
        Self { set: BTreeMap::new(), stream: vec![0] }
    }

    pub fn insert(&mut self, value: &[u8]) -> u32 {
        if value.is_empty() {
            return 0;
        }

        let pos = self.stream.len();
        let mut insert = false;

        let pos = *self.set.entry(value.to_vec()).or_insert_with(|| {
            insert = true;
            pos
        });

        if insert {
            self.stream.extend_from_slice(value);
        }

        pos as _
    }

    pub fn into_stream(mut self) -> Vec<u8> {
        self.stream.resize(round(self.stream.len(), 4), 0);
        self.stream
    }
}

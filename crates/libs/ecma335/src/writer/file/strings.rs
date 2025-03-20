use super::*;

pub struct Strings {
    map: HashMap<String, u32>,
    stream: Vec<u8>,
}

impl Default for Strings {
    fn default() -> Self {
        Self {
            map: Default::default(),
            stream: vec![0], // the empty string
        }
    }
}

impl Strings {
    pub fn insert(&mut self, value: &str) -> u32 {
        if value.is_empty() {
            return 0;
        }

        if let Some(pos) = self.map.get(value) {
            return *pos;
        }

        let pos = self.stream.len().try_into().unwrap();
        self.map.insert(value.to_string(), pos);
        self.stream.extend_from_slice(value.as_bytes());
        self.stream.push(0);
        pos
    }

    pub fn into_stream(self) -> Vec<u8> {
        self.stream.into_stream()
    }
}

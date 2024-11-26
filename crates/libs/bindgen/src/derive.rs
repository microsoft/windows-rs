use super::*;

pub struct Derive(HashMap<TypeName, Vec<String>>);

impl Derive {
    pub fn new(reader: &'static Reader, derive: &[&str]) -> Self {
        let mut map = HashMap::new();

        for derive in derive {
            let Some((name, derive)) = derive.split_once('=') else {
                panic!("invalid `--derive` must be `<type name>=Comma,Separated,List");
            };

            let type_name = reader.get_type_name(name);
            let derive = derive.split(',').filter_map(|derive| (!derive.is_empty()).then(||derive.to_string())).collect();
            map.insert(type_name, derive);
        }

        Self(map)
    }

    pub fn get(&self, type_name: TypeName) -> impl Iterator<Item = String> + '_ {
        self.0.get(&type_name).into_iter().flatten().cloned()
    }
}

use super::*;

pub struct Derive(HashMap<TypeName<'static>, Vec<String>>);

impl Derive {
    pub fn new(reader: &'static Reader, derive: &[&str]) -> Self {
        let mut map = HashMap::new();

        for derive in derive.iter().copied() {
            let Some((name, derive)) = derive.split_once('=') else {
                invalid_derive();
            };

            let type_name = reader.get_type_name(name);
            let derive = derive.split(',').map(|derive| derive.to_string()).collect();

            // TODO: check for duplicates?
            map.insert(type_name, derive);
        }

        Self(map)
    }

    pub fn get<'a>(&'a self, type_name: TypeName<'a>) -> impl Iterator<Item = String> + 'a {
        self.0.get(&type_name).into_iter().flatten().cloned()
    }

    // pub fn insert(&mut self, arg: &str) {
    //     if let Some((name, derive)) = arg.split_once('=') {
    //         derive
    //     }
    // }
}

fn invalid_derive() -> ! {
    panic!("invalid `--derive` must be `<type name>=Comma,Separated,List");
}

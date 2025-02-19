use super::*;

#[track_caller]
fn invalid_reference() -> ! {
    panic!("`--reference` must be `<crate>,<full/flat/skip-root>,<type name>");
}

pub struct ReferenceStage {
    name: String,
    style: ReferenceStyle,
    path: String,
}

impl Default for ReferenceStage {
    fn default() -> Self {
        Self {
            name: String::new(),
            style: ReferenceStyle::Full,
            path: String::new(),
        }
    }
}

impl ReferenceStage {
    #[track_caller]
    pub fn parse(arg: &str) -> Self {
        let arg: Vec<_> = arg.split(',').collect();

        if arg.len() != 3 {
            invalid_reference();
        }

        Self {
            name: arg[0].to_string(),
            style: ReferenceStyle::parse(arg[1]),
            path: arg[2].to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReferenceStyle {
    Full,
    Flat,
    SkipRoot, // used by windows and windows-sys crates
}

impl ReferenceStyle {
    #[track_caller]
    fn parse(arg: &str) -> Self {
        match arg {
            "full" => Self::Full,
            "flat" => Self::Flat,
            "skip-root" => Self::SkipRoot,
            _ => invalid_reference(),
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub name: String,          // crate name like "windows"
    pub types: TypeMap,        // what this reference provides
    pub style: ReferenceStyle, // how to generate the type path
}

#[derive(Debug, Default)]
pub struct References(Vec<Reference>);

impl References {
    pub fn new(reader: &Reader, stage: Vec<ReferenceStage>) -> Self {
        Self(
            stage
                .into_iter()
                .map(|stage| {
                    let filter = Filter::new(reader, &[&stage.path], &[]);
                    let types = TypeMap::filter(reader, &filter, &References::default());

                    Reference {
                        name: stage.name,
                        style: stage.style,
                        types,
                    }
                })
                .collect(),
        )
    }

    pub fn contains(&self, name: TypeName) -> Option<&Reference> {
        self.0
            .iter()
            .find(|reference| reference.types.contains_key(&name))
    }
}

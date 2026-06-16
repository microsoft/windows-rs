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
    pub fn parse(mut arg: &str) -> Self {
        if arg == "windows" {
            arg = "windows,skip-root,Windows";
        }

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

    pub fn new(name: &str, style: ReferenceStyle, path: &str) -> Self {
        Self {
            name: name.to_string(),
            style,
            path: path.to_string(),
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
    pub filter: Filter,        // what this reference provides
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
                    // Reference paths use dots (e.g. "Windows.Win32.Foundation.WIN32_ERROR")
                    // — convert to :: syntax for the new parser.
                    let path = stage.path.replace('.', "::");
                    let entries = filter_parser::parse_filter_entry(&path);
                    let resolved = filter_parser::resolve_entries(reader, &entries);
                    let filter = Filter::from_resolved(reader, &resolved, false);

                    Reference {
                        name: stage.name,
                        style: stage.style,
                        filter,
                    }
                })
                .collect(),
        )
    }

    pub fn contains(&self, name: TypeName) -> Option<&Reference> {
        self.0
            .iter()
            .find(|reference| reference.filter.includes_type_name(name).is_some())
    }

    pub fn matching_rule(&self, name: TypeName) -> Option<&str> {
        self.0
            .iter()
            .find_map(|reference| reference.filter.includes_type_name(name))
    }
}

use super::*;

// --reference name=windows,style=skip-root,path=Windows
// --reference name=windows-sys,style=skip-root,path=Windows

// --reference windows-numerics:flat:Windows.Foundation.Numerics
// --reference dia,full

fn invalid_reference() -> ! {
    panic!(
        "invalid `-reference` must be `name=<crate>,style=<full/flat/skip-root>,path=<type name>"
    );
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
    pub fn parse(arg: &str) -> Self {
        let mut result = ReferenceStage::default();

        for pair in arg.split(',') {
            match pair.split_once('=') {
                Some(("name", value)) => result.name = value.to_string(),
                Some(("style", value)) => result.style = ReferenceStyle::parse(value),
                Some(("path", value)) => result.path = value.to_string(),
                _ => invalid_reference(),
            }
        }

        if result.name.is_empty() || result.path.is_empty() {
            invalid_reference();
        }

        result
    }
}

#[derive(Debug, PartialEq)]
pub enum ReferenceStyle {
    Full,
    Flat,
    SkipRoot, // used by windows and windows-sys crates
}

impl ReferenceStyle {
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
    pub name: String,           // crate name like "windows"
    pub includes: Dependencies, // what this reference provides
    pub style: ReferenceStyle,  // how to generate the type path
}

#[derive(Debug)]
pub struct References(Vec<Reference>);

impl References {
    pub fn new(reader: &'static Reader, stage: Vec<ReferenceStage>) -> Self {
        Self(
            stage
                .into_iter()
                .map(|stage| {
                    // TODO: does this validate the path?
                    let filter = Filter::new(reader, &[&stage.path], &[]);
                    let includes = Dependencies::filter(reader, &filter);

                    Reference {
                        name: stage.name,
                        style: stage.style,
                        includes,
                    }
                })
                .collect(),
        )
    }

    pub fn contains(&self, name: TypeName<'_>) -> Option<&Reference> {
        self.0
            .iter()
            .find(|reference| reference.includes.contains(name))
    }
}

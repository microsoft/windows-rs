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

#[derive(Clone, Copy)]
enum DefaultReferenceName {
    Static(&'static str),
    Win32ResultOrCore,
}

impl DefaultReferenceName {
    fn resolve(self, specific_deps: bool) -> &'static str {
        match self {
            Self::Static(name) => name,
            Self::Win32ResultOrCore => {
                if specific_deps {
                    "windows_result"
                } else {
                    "windows_core"
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct DefaultReferenceGroup {
    required_namespace: &'static str,
    name: DefaultReferenceName,
    paths: &'static [&'static str],
}

const DEFAULT_REFERENCE_GROUPS: &[DefaultReferenceGroup] = &[
    DefaultReferenceGroup {
        required_namespace: "Windows.Foundation",
        name: DefaultReferenceName::Static("windows_future"),
        paths: &["Windows.Foundation.Async*", "Windows.Foundation.IAsync*"],
    },
    DefaultReferenceGroup {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        paths: &[
            "Windows.Foundation.Collections.CollectionChange",
            "Windows.Foundation.Collections.IIterable",
            "Windows.Foundation.Collections.IIterator",
            "Windows.Foundation.Collections.IKeyValuePair",
            "Windows.Foundation.Collections.IMap",
            "Windows.Foundation.Collections.IMapChangedEventArgs",
            "Windows.Foundation.Collections.IMapView",
            "Windows.Foundation.Collections.IObservableMap",
            "Windows.Foundation.Collections.IObservableVector",
            "Windows.Foundation.Collections.IVector",
            "Windows.Foundation.Collections.IVectorChangedEventArgs",
            "Windows.Foundation.Collections.IVectorView",
            "Windows.Foundation.Collections.MapChangedEventHandler",
            "Windows.Foundation.Collections.VectorChangedEventHandler",
        ],
    },
    DefaultReferenceGroup {
        required_namespace: "Windows.Foundation",
        name: DefaultReferenceName::Static("windows_reference"),
        paths: &["Windows.Foundation.IReference"],
    },
    DefaultReferenceGroup {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        paths: &[
            "Windows.Foundation.Numerics.Matrix3x2",
            "Windows.Foundation.Numerics.Matrix4x4",
            "Windows.Foundation.Numerics.Vector2",
            "Windows.Foundation.Numerics.Vector3",
            "Windows.Foundation.Numerics.Vector4",
        ],
    },
    DefaultReferenceGroup {
        required_namespace: "Windows.Win32.Foundation",
        name: DefaultReferenceName::Win32ResultOrCore,
        paths: &[
            "Windows.Win32.Foundation.WIN32_ERROR",
            "Windows.Win32.Foundation.NTSTATUS",
            "Windows.Win32.System.Rpc.RPC_STATUS",
        ],
    },
];

pub fn default_reference_stages(reader: &Reader, specific_deps: bool) -> Vec<ReferenceStage> {
    let mut references = Vec::with_capacity(
        DEFAULT_REFERENCE_GROUPS
            .iter()
            .map(|group| group.paths.len())
            .sum(),
    );

    // Reverse iteration preserves the historical "insert at front" ordering.
    for group in DEFAULT_REFERENCE_GROUPS.iter().rev() {
        if reader.contains_key(group.required_namespace) {
            for path in group.paths.iter().rev() {
                references.push(ReferenceStage::new(
                    group.name.resolve(specific_deps),
                    ReferenceStyle::Flat,
                    path,
                ));
            }
        }
    }

    references
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
                    let filter = Filter::new(reader, &[&stage.path], &[]);

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

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
struct DefaultReference {
    required_namespace: &'static str,
    name: DefaultReferenceName,
    path: &'static str,
}

const DEFAULT_REFERENCES: &[DefaultReference] = &[
    DefaultReference {
        required_namespace: "Windows.Foundation",
        name: DefaultReferenceName::Static("windows_future"),
        path: "Windows.Foundation.Async*",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation",
        name: DefaultReferenceName::Static("windows_future"),
        path: "Windows.Foundation.IAsync*",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.CollectionChange",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IIterable",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IIterator",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IKeyValuePair",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IMap",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IMapChangedEventArgs",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IMapView",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IObservableMap",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IObservableVector",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IVector",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IVectorChangedEventArgs",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.IVectorView",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.MapChangedEventHandler",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Collections",
        name: DefaultReferenceName::Static("windows_collections"),
        path: "Windows.Foundation.Collections.VectorChangedEventHandler",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation",
        name: DefaultReferenceName::Static("windows_reference"),
        path: "Windows.Foundation.IReference",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        path: "Windows.Foundation.Numerics.Matrix3x2",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        path: "Windows.Foundation.Numerics.Matrix4x4",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        path: "Windows.Foundation.Numerics.Vector2",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        path: "Windows.Foundation.Numerics.Vector3",
    },
    DefaultReference {
        required_namespace: "Windows.Foundation.Numerics",
        name: DefaultReferenceName::Static("windows_numerics"),
        path: "Windows.Foundation.Numerics.Vector4",
    },
    DefaultReference {
        required_namespace: "Windows.Win32.Foundation",
        name: DefaultReferenceName::Win32ResultOrCore,
        path: "Windows.Win32.Foundation.WIN32_ERROR",
    },
    DefaultReference {
        required_namespace: "Windows.Win32.Foundation",
        name: DefaultReferenceName::Win32ResultOrCore,
        path: "Windows.Win32.Foundation.NTSTATUS",
    },
    DefaultReference {
        required_namespace: "Windows.Win32.Foundation",
        name: DefaultReferenceName::Win32ResultOrCore,
        path: "Windows.Win32.System.Rpc.RPC_STATUS",
    },
];

pub fn default_reference_stages(reader: &Reader, specific_deps: bool) -> Vec<ReferenceStage> {
    let mut references = Vec::new();

    for reference in DEFAULT_REFERENCES {
        if reader.contains_key(reference.required_namespace) {
            references.insert(
                0,
                ReferenceStage::new(
                    reference.name.resolve(specific_deps),
                    ReferenceStyle::Flat,
                    reference.path,
                ),
            );
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

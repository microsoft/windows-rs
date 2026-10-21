pub const MACRO_SUPPORT_HEADERS: &[&str] = &["driverspecs.h", "rpcsal.h", "specstrings.h"];

pub const LEAN_GUID_HEADERS: &[(&str, &str)] =
    &[("winioctl.h", "winioctl.h"), ("winscard.h", "winsmcrd.h")];

pub struct Context {
    pub header: &'static str,
    pub namespaces: bool,
    pub preload: &'static str,
    pub declaration: &'static str,
    pub definition: &'static str,
}

const CONTEXTS: &[Context] = &[
    Context {
        header: "devicetopology.h",
        namespaces: false,
        // Load KS before main headers can select devicetopology's fallback types.
        preload: "#include <ks.h>\n",
        declaration: "#include <ks.h>\n#define _KS_\n",
        definition: "#define _KS_\n",
    },
    Context {
        header: "memorybuffer.h",
        namespaces: true,
        preload: "",
        declaration: "",
        definition: "",
    },
    Context {
        header: "robuffer.h",
        namespaces: true,
        preload: "",
        declaration: "",
        definition: "",
    },
    Context {
        header: "windows.ui.composition.interop.h",
        namespaces: true,
        preload: "",
        // The interop header only forward-declares its desktop WinRT dependency.
        declaration: "#include <windows.ui.composition.desktop.h>\n",
        definition: "",
    },
];

pub fn get(header: &str) -> Option<&'static Context> {
    CONTEXTS.iter().find(|context| context.header == header)
}

pub fn namespace_headers(headers: &[String]) -> Vec<String> {
    headers
        .iter()
        .filter(|header| get(header).is_some_and(|context| context.namespaces))
        .cloned()
        .collect()
}

pub fn preload(headers: &[String]) -> String {
    CONTEXTS
        .iter()
        .filter(|context| headers.iter().any(|header| header == context.header))
        .map(|context| context.preload)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BTreeSet, headers};

    #[test]
    fn configured_headers_are_unique_workload_members() {
        let mut names = BTreeSet::new();
        for context in CONTEXTS {
            assert!(names.insert(context.header), "{}", context.header);
            assert!(
                headers::HEADERS.contains(&context.header)
                    || headers::SATELLITE_HEADERS.contains(&context.header),
                "{}",
                context.header
            );
        }
    }

    #[test]
    fn namespace_selection_does_not_select_support_headers() {
        let selected = [
            "robuffer.h",
            "devicetopology.h",
            "windows.ui.composition.interop.h",
            "windows.ui.composition.desktop.h",
            "memorybuffer.h",
        ]
        .map(String::from);
        assert_eq!(
            namespace_headers(&selected),
            [
                "robuffer.h",
                "windows.ui.composition.interop.h",
                "memorybuffer.h"
            ]
        );
        assert!(namespace_headers(&["windows.h".into()]).is_empty());
        assert_eq!(
            preload(&["devicetopology.h".into(), "devicetopology.h".into()]),
            "#include <ks.h>\n"
        );
        assert!(preload(&["robuffer.h".into()]).is_empty());
    }
}

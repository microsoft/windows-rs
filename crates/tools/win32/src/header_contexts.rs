pub const MACRO_SUPPORT_HEADERS: &[&str] = &["driverspecs.h", "rpcsal.h", "specstrings.h"];

pub const LEAN_GUID_HEADERS: &[(&str, &str)] =
    &[("winioctl.h", "winioctl.h"), ("winscard.h", "winsmcrd.h")];

pub struct Context {
    pub header: &'static str,
    pub preload: &'static str,
    pub declaration: &'static str,
    pub definition: &'static str,
}

const CONTEXTS: &[Context] = &[
    Context {
        header: "devicetopology.h",
        // Load KS before main headers can select devicetopology's fallback types.
        preload: "#include <ks.h>\n",
        declaration: "#include <ks.h>\n#define _KS_\n",
        definition: "#define _KS_\n",
    },
    Context {
        header: "windows.ui.composition.interop.h",
        preload: "",
        // The interop header only forward-declares its desktop WinRT dependency.
        declaration: "#include <windows.ui.composition.desktop.h>\n",
        definition: "",
    },
    Context {
        header: "gdiplus.h",
        preload: "#define GDIPVER 0x0110\n",
        declaration: "#include <objidl.h>\n",
        definition: "#include <objidl.h>\n",
    },
];

pub fn get(header: &str) -> Option<&'static Context> {
    CONTEXTS.iter().find(|context| context.header == header)
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
    fn preload_selection() {
        assert_eq!(
            preload(&["devicetopology.h".into(), "devicetopology.h".into()]),
            "#include <ks.h>
"
        );
        assert!(preload(&["windows.h".into()]).is_empty());
        assert_eq!(
            preload(&["gdiplus.h".into()]),
            "#define GDIPVER 0x0110
"
        );
    }
}

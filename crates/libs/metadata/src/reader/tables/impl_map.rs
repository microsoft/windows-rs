use super::*;

/// Invalid `#ordinal` syntax in a P/Invoke entry point.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImportOrdinalError;

impl std::fmt::Display for ImportOrdinalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expected # followed by an unsigned 16-bit decimal ordinal")
    }
}

impl std::error::Error for ImportOrdinalError {}

pub fn parse_import_ordinal(name: &str) -> Result<Option<u16>, ImportOrdinalError> {
    let Some(value) = name.strip_prefix('#') else {
        return Ok(None);
    };
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ImportOrdinalError);
    }
    value.parse().map(Some).map_err(|_| ImportOrdinalError)
}

#[test]
fn import_ordinals() {
    for (name, expected) in [
        ("Export", None),
        ("Export#17", None),
        ("#0", Some(0)),
        ("#17", Some(17)),
        ("#00017", Some(17)),
        ("#65535", Some(u16::MAX)),
    ] {
        assert_eq!(parse_import_ordinal(name), Ok(expected));
    }
    for name in [
        "#", "#-1", "#+1", "# 1", "#1 ", "#0x11", "#65536", "#Export",
    ] {
        assert_eq!(parse_import_ordinal(name), Err(ImportOrdinalError));
    }
}

impl std::fmt::Debug for ImplMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ImplMap").field(&self.import_name()).finish()
    }
}

impl<'a> ImplMap<'a> {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn import_name(&self) -> &'a str {
        self.str(2)
    }

    pub fn import_ordinal(&self) -> Result<Option<u16>, ImportOrdinalError> {
        parse_import_ordinal(self.import_name())
    }

    pub fn import_scope(&self) -> ModuleRef<'a> {
        self.row(3)
    }
}

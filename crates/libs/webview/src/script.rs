/// Identifies a script registered on document creation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptId(pub(crate) String);

impl ScriptId {
    /// Returns the underlying WebView2 script identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

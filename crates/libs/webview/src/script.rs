/// Identifies a script registered with
/// [`WebView::add_script_to_execute_on_document_created`]. Pass it to
/// [`WebView::remove_script_to_execute_on_document_created`] to unregister the
/// script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptId(pub(crate) String);

impl ScriptId {
    /// Returns the underlying WebView2 script identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

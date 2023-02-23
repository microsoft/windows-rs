impl std::fmt::Debug for windows::core::IInspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents.
        let name =
            <windows::core::IInspectable as windows::core::Interface>::cast::<IStringable>(self)
                .and_then(|s| s.ToString())
                .or_else(|_| self.GetRuntimeClassName())
                .unwrap_or_default();
        write!(f, "\"{}\"", name)
    }
}

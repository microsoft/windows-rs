fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen::builder()
        .output(&bindings)
        .flat()
        .filters([
            // COM plumbing
            "CoInitializeEx",
            "CoCreateInstance",
            "CoTaskMemFree",
            "COINIT_MULTITHREADED",
            "CLSCTX_ALL",
            // Spell checking
            "SpellCheckerFactory",
            "ISpellCheckerFactory",
            "ISpellChecker",
            "IEnumSpellingError",
            "ISpellingError",
            "CORRECTIVE_ACTION",
            // Enum *values* must be filtered explicitly — naming the enum type alone
            // emits the newtype but none of its constants, which silently turns
            // `match` arms into bindings.
            "CORRECTIVE_ACTION_DELETE",
            "CORRECTIVE_ACTION_REPLACE",
            "CORRECTIVE_ACTION_GET_SUGGESTIONS",
            // Enumerator returned by `ISpellChecker::Suggest`. Referenced by the
            // methods above; without it those methods are silently dropped.
            "IEnumString",
        ])
        .write();
}

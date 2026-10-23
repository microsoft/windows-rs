use super::*;

pub fn selection() -> Result<BTreeMap<Target, BTreeSet<String>>, String> {
    let mut selected = BTreeMap::<Target, BTreeSet<String>>::new();
    for line in include_str!("function-aliases.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        let [mask, name] = fields.as_slice() else {
            return Err(format!("invalid function alias selection: {line:?}"));
        };
        let mask: u8 = mask
            .parse()
            .map_err(|_| format!("invalid alias target mask: {line:?}"))?;
        if mask == 0
            || mask & !7 != 0
            || !name.bytes().enumerate().all(|(i, byte)| {
                byte == b'_' || byte.is_ascii_alphabetic() || (i != 0 && byte.is_ascii_digit())
            })
        {
            return Err(format!("invalid function alias selection: {line:?}"));
        }
        for (target, bit) in [(Target::X86, 1), (Target::X64, 2), (Target::Arm64, 4)] {
            if mask & bit != 0 && !selected.entry(target).or_default().insert((*name).into()) {
                return Err(format!("duplicate function alias selection: {line:?}"));
            }
        }
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_compatibility_selection() {
        let selected = selection().unwrap();
        for name in [
            "EnumProcesses",
            "GetProcessMemoryInfo",
            "GetWindowLongPtrW",
            "UrlIs",
        ] {
            assert!(selected[&Target::X86].contains(name), "{name}");
        }
        for name in [
            "CreateFile",
            "CreateEvent",
            "AnsiLower",
            "AccFree",
            "AVIFileClose",
        ] {
            assert!(
                selected.values().all(|names| !names.contains(name)),
                "{name}"
            );
        }
        assert!(!selected[&Target::X64].contains("GetWindowLongPtrW"));
        assert!(!selected[&Target::X64].contains("SymGetModuleInfo"));
    }
}

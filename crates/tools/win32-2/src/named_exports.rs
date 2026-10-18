use super::*;

// Reviewed public entry points whose pinned SDK import records select ordinals.
// Names are publication policy, not names inferred by undecorating linker symbols.
// (export name, DLL, SDK ordinal, x86 linker symbol).
pub const EXPORTS: &[(&str, &str, u16, &str)] = &[
    ("WSAStartup", "ws2_32.dll", 115, "_WSAStartup@8"),
    ("WSACleanup", "ws2_32.dll", 116, "_WSACleanup@0"),
    ("socket", "ws2_32.dll", 23, "_socket@12"),
    ("bind", "ws2_32.dll", 2, "_bind@12"),
    ("listen", "ws2_32.dll", 13, "_listen@8"),
    ("accept", "ws2_32.dll", 1, "_accept@12"),
    ("connect", "ws2_32.dll", 4, "_connect@12"),
    ("send", "ws2_32.dll", 19, "_send@16"),
    ("recv", "ws2_32.dll", 16, "_recv@16"),
    ("closesocket", "ws2_32.dll", 3, "_closesocket@4"),
    (
        "D3D12CreateDevice",
        "d3d12.dll",
        101,
        "_D3D12CreateDevice@16",
    ),
    (
        "D3D12GetDebugInterface",
        "d3d12.dll",
        102,
        "_D3D12GetDebugInterface@8",
    ),
    (
        "D2D1MakeRotateMatrix",
        "d2d1.dll",
        2,
        "_D2D1MakeRotateMatrix@16",
    ),
    (
        "D2D1MakeSkewMatrix",
        "d2d1.dll",
        3,
        "_D2D1MakeSkewMatrix@20",
    ),
    (
        "D2D1IsMatrixInvertible",
        "d2d1.dll",
        4,
        "_D2D1IsMatrixInvertible@4",
    ),
    ("D2D1InvertMatrix", "d2d1.dll", 5, "_D2D1InvertMatrix@4"),
    ("D2D1CreateDevice", "d2d1.dll", 7, "_D2D1CreateDevice@12"),
    (
        "D2D1CreateDeviceContext",
        "d2d1.dll",
        8,
        "_D2D1CreateDeviceContext@12",
    ),
    ("ILClone", "shell32.dll", 18, "_ILClone@4"),
    ("ILFree", "shell32.dll", 155, "_ILFree@4"),
    (
        "ADsBuildEnumerator",
        "activeds.dll",
        4,
        "_ADsBuildEnumerator@8",
    ),
    (
        "ADsBuildVarArrayInt",
        "activeds.dll",
        8,
        "_ADsBuildVarArrayInt@12",
    ),
    (
        "ADsBuildVarArrayStr",
        "activeds.dll",
        7,
        "_ADsBuildVarArrayStr@12",
    ),
    (
        "ADsDecodeBinaryData",
        "activeds.dll",
        24,
        "_ADsDecodeBinaryData@12",
    ),
    (
        "ADsEncodeBinaryData",
        "activeds.dll",
        20,
        "_ADsEncodeBinaryData@12",
    ),
    (
        "ADsEnumerateNext",
        "activeds.dll",
        6,
        "_ADsEnumerateNext@16",
    ),
    (
        "ADsFreeEnumerator",
        "activeds.dll",
        5,
        "_ADsFreeEnumerator@4",
    ),
    ("ADsGetLastError", "activeds.dll", 13, "_ADsGetLastError@20"),
    ("ADsGetObject", "activeds.dll", 3, "_ADsGetObject@12"),
    ("ADsOpenObject", "activeds.dll", 9, "_ADsOpenObject@24"),
    ("ADsSetLastError", "activeds.dll", 12, "_ADsSetLastError@12"),
    (
        "AdsFreeAdsValues",
        "activeds.dll",
        23,
        "_AdsFreeAdsValues@8",
    ),
    (
        "AdsTypeToPropVariant",
        "activeds.dll",
        22,
        "_AdsTypeToPropVariant@12",
    ),
    ("AllocADsMem", "activeds.dll", 14, "_AllocADsMem@4"),
    ("AllocADsStr", "activeds.dll", 17, "_AllocADsStr@4"),
    (
        "BinarySDToSecurityDescriptor",
        "activeds.dll",
        29,
        "_BinarySDToSecurityDescriptor@24",
    ),
    ("FreeADsMem", "activeds.dll", 15, "_FreeADsMem@4"),
    ("FreeADsStr", "activeds.dll", 18, "_FreeADsStr@4"),
    (
        "PropVariantToAdsType",
        "activeds.dll",
        21,
        "_PropVariantToAdsType@16",
    ),
    ("ReallocADsMem", "activeds.dll", 16, "_ReallocADsMem@12"),
    ("ReallocADsStr", "activeds.dll", 19, "_ReallocADsStr@8"),
    (
        "SecurityDescriptorToBinarySD",
        "activeds.dll",
        30,
        "_SecurityDescriptorToBinarySD@40",
    ),
];

pub fn resolve(
    target: Target,
    import: &windows_rdl::implib::Import,
) -> Result<Option<&'static str>, String> {
    let Some(&(name, library, ordinal, _)) = EXPORTS.iter().find(|&&(name, _, _, x86)| {
        import.symbol == if target == Target::X86 { x86 } else { name }
    }) else {
        return Ok(None);
    };
    if import.machine != import_target(target).1
        || import.kind != windows_rdl::implib::ImportKind::Code
        || !import.dll.eq_ignore_ascii_case(library)
        || import.name != windows_rdl::implib::ImportName::Ordinal(ordinal)
    {
        return Err(format!(
            "{target:?} SDK import for `{name}` conflicts with its reviewed named-export policy: {import:?}"
        ));
    }
    Ok(Some(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_rdl::implib::{Import, ImportKind, ImportName};

    #[test]
    fn exact_evidence() {
        for target in [Target::X86, Target::X64, Target::Arm64] {
            for &(name, dll, ordinal, x86) in EXPORTS {
                let import = Import {
                    machine: import_target(target).1,
                    kind: ImportKind::Code,
                    symbol: if target == Target::X86 { x86 } else { name }.into(),
                    dll: dll.into(),
                    name: ImportName::Ordinal(ordinal),
                };
                assert_eq!(resolve(target, &import).unwrap(), Some(name));
                for changed in [
                    Import {
                        machine: 0,
                        ..import.clone()
                    },
                    Import {
                        kind: ImportKind::Data,
                        ..import.clone()
                    },
                    Import {
                        dll: "other.dll".into(),
                        ..import.clone()
                    },
                    Import {
                        name: ImportName::Ordinal(ordinal + 1),
                        ..import.clone()
                    },
                ] {
                    assert!(resolve(target, &changed).is_err());
                }
                let unknown = Import {
                    symbol: "unreviewed".into(),
                    ..import
                };
                assert_eq!(resolve(target, &unknown).unwrap(), None);
            }
        }
    }
}

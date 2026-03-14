use windows_metadata::writer;
use windows_metadata::Type;

/// The midlrt namespace GUID {e72a134c-baf7-4dd3-b542-77848e87b138} in network (big-endian) byte
/// order. This is used as the UUID v5 namespace for WinRT interface GUID derivation.
const MIDLRT_NAMESPACE: [u8; 16] = [
    0xe7, 0x2a, 0x13, 0x4c, 0xba, 0xf7, 0x4d, 0xd3, 0xb5, 0x42, 0x77, 0x84, 0x8e, 0x87, 0xb1, 0x38,
];

/// Computes a deterministic interface GUID from an interface string using RFC 4122 UUID v5
/// (SHA-1 name-based UUID) with the midlrt namespace.
///
/// Returns `(data1, data2, data3, data4)` suitable for writing a `GuidAttribute`.
pub fn guid_from_interface_string(interface_string: &str) -> (u32, u16, u16, [u8; 8]) {
    let hash = sha1(&MIDLRT_NAMESPACE, interface_string.as_bytes());

    let data1 = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);
    let data2 = u16::from_be_bytes([hash[4], hash[5]]);
    // Set version = 5 in the high nibble of data3
    let data3 = (u16::from_be_bytes([hash[6], hash[7]]) & 0x0fff) | 0x5000;
    // Set RFC 4122 variant in the high bits of data4[0]
    let data4 = [
        (hash[8] & 0x3f) | 0x80,
        hash[9],
        hash[10],
        hash[11],
        hash[12],
        hash[13],
        hash[14],
        hash[15],
    ];

    (data1, data2, data3, data4)
}

/// Derives a GUID from the interface/delegate shape and emits a `GuidAttribute` on `target`.
///
/// Builds the interface string from `namespace`, `name`, and `methods`, computes the UUID v5
/// GUID, and writes the attribute to `output`. Shared by both `interface.rs` and `delegate.rs`.
pub fn derive_and_emit_guid(
    output: &mut writer::File,
    target: writer::HasAttribute,
    namespace: &str,
    name: &str,
    methods: &[(&str, &[Type], &Type)],
) {
    let interface_string = build_interface_string(namespace, name, methods);
    let (data1, data2, data3, data4) = guid_from_interface_string(&interface_string);
    emit_guid_attribute(output, target, data1, data2, data3, data4);
}

/// Emits a `GuidAttribute` with the given GUID components to `target` in `output`.
pub fn emit_guid_attribute(
    output: &mut writer::File,
    target: writer::HasAttribute,
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
) {
    let guid_typeref = output.TypeRef("Windows.Foundation.Metadata", "GuidAttribute");

    let signature = windows_metadata::Signature {
        flags: windows_metadata::MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![
            Type::U32,
            Type::U16,
            Type::U16,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
        ],
    };

    let ctor = output.MemberRef(
        ".ctor",
        &signature,
        writer::MemberRefParent::TypeRef(guid_typeref),
    );

    let val = |v: windows_metadata::Value| (String::new(), v);
    output.Attribute(
        target,
        writer::AttributeType::MemberRef(ctor),
        &[
            val(windows_metadata::Value::U32(data1)),
            val(windows_metadata::Value::U16(data2)),
            val(windows_metadata::Value::U16(data3)),
            val(windows_metadata::Value::U8(data4[0])),
            val(windows_metadata::Value::U8(data4[1])),
            val(windows_metadata::Value::U8(data4[2])),
            val(windows_metadata::Value::U8(data4[3])),
            val(windows_metadata::Value::U8(data4[4])),
            val(windows_metadata::Value::U8(data4[5])),
            val(windows_metadata::Value::U8(data4[6])),
            val(windows_metadata::Value::U8(data4[7])),
        ],
    );
}

/// Builds the interface string for a method-based interface or delegate.
///
/// Format: `"namespace.Name:HRESULT Method1(param1,param2,...);HRESULT Method2(...);..."`
///
/// For empty interfaces (no methods): `"namespace.Name:"`
///
/// Each type is encoded using the literal `Type` variant name (e.g. `I32`, `Bool`,
/// `PtrMut(I32,1)`, `Array(Bool)`), matching the `windows-metadata` type one-to-one.
pub fn build_interface_string(
    namespace: &str,
    name: &str,
    methods: &[(&str, &[Type], &Type)],
) -> String {
    let mut s = String::new();
    s.push_str(namespace);
    s.push('.');
    s.push_str(name);
    s.push(':');

    for (method_name, param_types, return_type) in methods {
        s.push_str("HRESULT ");
        s.push_str(method_name);
        s.push('(');

        let mut parts: Vec<String> = param_types.iter().map(type_to_string).collect();
        parts.push(type_to_string(return_type));
        s.push_str(&parts.join(","));

        s.push(')');
        s.push(';');
    }

    s
}

/// Converts a `metadata::Type` to its literal variant-name representation, matching the
/// `windows-metadata` `Type` enum one-to-one.
pub fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Void => "Void".to_string(),
        Type::Bool => "Bool".to_string(),
        Type::Char => "Char".to_string(),
        Type::I8 => "I8".to_string(),
        Type::U8 => "U8".to_string(),
        Type::I16 => "I16".to_string(),
        Type::U16 => "U16".to_string(),
        Type::I32 => "I32".to_string(),
        Type::U32 => "U32".to_string(),
        Type::I64 => "I64".to_string(),
        Type::U64 => "U64".to_string(),
        Type::F32 => "F32".to_string(),
        Type::F64 => "F64".to_string(),
        Type::ISize => "ISize".to_string(),
        Type::USize => "USize".to_string(),
        Type::String => "String".to_string(),
        Type::Object => "Object".to_string(),
        Type::Generic(name, index) => format!("Generic({name},{index})"),
        Type::Name(tn) => {
            if tn.generics.is_empty() {
                format!("{}.{}", tn.namespace, tn.name)
            } else {
                let args: Vec<String> = tn.generics.iter().map(type_to_string).collect();
                format!("{}.{}<{}>", tn.namespace, tn.name, args.join(","))
            }
        }
        Type::PtrMut(inner, depth) => format!("PtrMut({},{})", type_to_string(inner), depth),
        Type::PtrConst(inner, depth) => format!("PtrConst({},{})", type_to_string(inner), depth),
        Type::RefMut(inner) => format!("RefMut({})", type_to_string(inner)),
        Type::RefConst(inner) => format!("RefConst({})", type_to_string(inner)),
        Type::Array(inner) => format!("Array({})", type_to_string(inner)),
        Type::ArrayFixed(inner, n) => format!("ArrayFixed({},{})", type_to_string(inner), n),
    }
}

/// A minimal runtime SHA-1 implementation (not const fn).
/// Takes a 16-byte prefix (namespace GUID in network byte order) and the name bytes,
/// and returns the 20-byte SHA-1 digest of their concatenation.
fn sha1(prefix: &[u8; 16], name: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

    // Build padded message: prefix || name || padding
    let total_len = prefix.len() + name.len();
    let bit_len = (total_len as u64) * 8;

    // Pad to a multiple of 64 bytes: append 0x80, then zeros, then 8-byte big-endian length
    let padded_len = {
        let raw = total_len + 1 + 8; // +1 for 0x80, +8 for length
        (raw + 63) & !63 // round up to multiple of 64
    };

    let mut msg = vec![0u8; padded_len];
    msg[..prefix.len()].copy_from_slice(prefix);
    msg[prefix.len()..prefix.len() + name.len()].copy_from_slice(name);
    msg[total_len] = 0x80;
    msg[padded_len - 8..].copy_from_slice(&bit_len.to_be_bytes());

    // Process each 64-byte chunk
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];

        // Fill first 16 words from chunk (big-endian)
        for (i, word) in w[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }

        // Expand to 80 words
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);

        for (i, &wi) in w.iter().enumerate() {
            let (f, k): (u32, u32) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut result = [0u8; 20];
    for (i, &word) in h.iter().enumerate() {
        result[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(interface_string: &str, expected: &str) {
        let (d1, d2, d3, d4) = guid_from_interface_string(interface_string);
        let actual = format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
        );
        assert_eq!(actual, expected, "interface_string = {interface_string:?}");
    }

    #[test]
    fn guid_empty_interface() {
        // Empty interfaces (no methods): hash of "namespace.Name:"
        check(
            "test_composable.IContainerVisualFactory:",
            "558b6180-1a65-5f01-8be2-2cc0b2034c0e",
        );
        check(
            "test_composable.IVisualFactory:",
            "1974545d-259f-553c-8ea0-e505f897df81",
        );
    }

    #[test]
    fn guid_simple_method() {
        // Methods with no parameters: hash of "namespace.Name:HRESULT Method();"
        check(
            "test_component.Nested.IThing:HRESULT Method();",
            "5448be22-9873-5ae6-9106-f6e8455d2fdd",
        );
        check(
            "test_activation.One.IMissing:HRESULT Method();",
            "ad54a92f-16de-537c-b6c0-5099534ee12e",
        );
    }

    #[test]
    fn build_interface_string_literal_types() {
        use windows_metadata::TypeName;

        // Single-arg generic: IIterable<I32> — literal variant-name encoding, no backtick
        let iter_ty = Type::Name(TypeName {
            namespace: "Windows.Foundation.Collections".to_string(),
            name: "IIterable".to_string(),
            generics: vec![Type::I32],
        });
        let single = build_interface_string(
            "Test",
            "ISingle",
            &[(
                "get_Items",
                &[Type::PtrMut(Box::new(iter_ty), 1)],
                &Type::Void,
            )],
        );
        assert_eq!(
            single,
            "Test.ISingle:HRESULT get_Items(PtrMut(Windows.Foundation.Collections.IIterable<I32>,1),Void);"
        );

        // Two-arg generic: IKeyValuePair<String,I32> — args joined with ","
        let kvp_ty = Type::Name(TypeName {
            namespace: "Windows.Foundation.Collections".to_string(),
            name: "IKeyValuePair".to_string(),
            generics: vec![Type::String, Type::I32],
        });
        let two_arg = build_interface_string(
            "Test",
            "ITwoArg",
            &[(
                "get_Pair",
                &[Type::PtrMut(Box::new(kvp_ty), 1)],
                &Type::Void,
            )],
        );
        assert_eq!(
            two_arg,
            "Test.ITwoArg:HRESULT get_Pair(PtrMut(Windows.Foundation.Collections.IKeyValuePair<String,I32>,1),Void);"
        );

        // Array param encoded as Array(inner), not expanded to UInt32 + T*
        let arr = build_interface_string(
            "Test",
            "IArr",
            &[("Fill", &[Type::Array(Box::new(Type::I32))], &Type::Void)],
        );
        assert_eq!(arr, "Test.IArr:HRESULT Fill(Array(I32),Void);");

        // Return type encoded literally without added *
        let ret = build_interface_string("Test", "IRet", &[("get_V", &[], &Type::I32)]);
        assert_eq!(ret, "Test.IRet:HRESULT get_V(I32);");

        // Pointer types
        let ptrs = build_interface_string(
            "Test",
            "IPtr",
            &[(
                "Method",
                &[
                    Type::PtrMut(Box::new(Type::I32), 2),
                    Type::PtrConst(Box::new(Type::I32), 1),
                    Type::RefMut(Box::new(Type::I32)),
                    Type::RefConst(Box::new(Type::I32)),
                ],
                &Type::Void,
            )],
        );
        assert_eq!(
            ptrs,
            "Test.IPtr:HRESULT Method(PtrMut(I32,2),PtrConst(I32,1),RefMut(I32),RefConst(I32),Void);"
        );
    }
}

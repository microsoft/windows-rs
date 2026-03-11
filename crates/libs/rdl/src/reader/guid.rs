use windows_metadata::Type;
use windows_metadata::writer;

/// The midlrt namespace GUID {e72a134c-baf7-4dd3-b542-77848e87b138} in network (big-endian) byte
/// order. This is used as the UUID v5 namespace for WinRT interface GUID derivation.
const MIDLRT_NAMESPACE: [u8; 16] = [
    0xe7, 0x2a, 0x13, 0x4c, 0xba, 0xf7, 0x4d, 0xd3, 0xb5, 0x42, 0x77, 0x84, 0x8e, 0x87, 0xb1,
    0x38,
];

/// Computes a deterministic WinRT interface GUID from an interface string using the midlrt
/// algorithm (RFC 4122 UUID v5 / SHA-1 name-based UUID).
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

/// Builds the WinRT interface string for a method-based interface or delegate.
///
/// Format: `"namespace.Name:HRESULT Method1(param1,param2,...);HRESULT Method2(...);..."`
///
/// For empty interfaces (no methods): `"namespace.Name:"`
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

        let mut first = true;
        for ty in *param_types {
            if !first {
                s.push(',');
            }
            first = false;
            s.push_str(&type_to_string(ty));
        }

        // Non-void return type becomes the last [out, retval] parameter with one extra pointer
        if !matches!(return_type, Type::Void) {
            if !first {
                s.push(',');
            }
            // Return types get one extra pointer level (the [out,retval] indirection)
            s.push_str(&type_to_string_extra(return_type, 1));
        }

        s.push(')');
        s.push(';');
    }

    s
}

/// Converts a `metadata::Type` to its WinRT interface string representation.
pub fn type_to_string(ty: &Type) -> String {
    type_to_string_extra(ty, 0)
}

/// Converts a `metadata::Type` to its WinRT interface string representation, appending
/// `extra_stars` additional pointer levels (used for return types).
pub fn type_to_string_extra(ty: &Type, extra_stars: usize) -> String {
    match ty {
        Type::Void => String::new(),
        Type::Bool => format!("Boolean{}", stars(extra_stars)),
        Type::Char => format!("Char16{}", stars(extra_stars)),
        Type::I8 => format!("Int8{}", stars(extra_stars)),
        Type::U8 => format!("UInt8{}", stars(extra_stars)),
        Type::I16 => format!("Int16{}", stars(extra_stars)),
        Type::U16 => format!("UInt16{}", stars(extra_stars)),
        Type::I32 => format!("Int32{}", stars(extra_stars)),
        Type::U32 => format!("UInt32{}", stars(extra_stars)),
        Type::I64 => format!("Int64{}", stars(extra_stars)),
        Type::U64 => format!("UInt64{}", stars(extra_stars)),
        Type::F32 => format!("Single{}", stars(extra_stars)),
        Type::F64 => format!("Double{}", stars(extra_stars)),
        Type::ISize => format!("IntPtr{}", stars(extra_stars)),
        Type::USize => format!("UIntPtr{}", stars(extra_stars)),
        Type::String => format!("String{}", stars(extra_stars)),
        Type::Object => format!("Object{}", stars(extra_stars)),
        Type::Generic(name, _) => format!("{name}{}", stars(extra_stars)),
        Type::Name(tn) => {
            let base = if tn.generics.is_empty() {
                format!("{}.{}", tn.namespace, tn.name)
            } else {
                // Backtick-N notation for generic types (e.g., IVector`1<Int32>)
                let args: Vec<String> = tn.generics.iter().map(type_to_string).collect();
                format!(
                    "{}.{}`{}<{}>",
                    tn.namespace,
                    tn.name,
                    tn.generics.len(),
                    args.join(",")
                )
            };
            format!("{base}{}", stars(extra_stars))
        }
        // Pointer types: the depth encodes the number of * levels
        Type::PtrMut(inner, depth) => {
            type_to_string_extra(inner, depth + extra_stars)
        }
        Type::PtrConst(inner, depth) => {
            // Const pointers use & suffix per the midlrt convention
            let base = type_to_string(inner);
            format!("{base}{}", ampersands(depth + extra_stars))
        }
        Type::RefMut(inner) => type_to_string_extra(inner, 1 + extra_stars),
        Type::RefConst(inner) => {
            let base = type_to_string(inner);
            format!("{base}{}", ampersands(1 + extra_stars))
        }
        // Arrays are not applicable for WinRT interface parameter type strings
        Type::Array(_) | Type::ArrayFixed(_, _) => String::new(),
    }
}

fn stars(n: usize) -> String {
    "*".repeat(n)
}

fn ampersands(n: usize) -> String {
    "&".repeat(n)
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
        // test_composable.IContainerVisualFactory (no methods)
        check(
            "test_composable.IContainerVisualFactory:",
            "558b6180-1a65-5f01-8be2-2cc0b2034c0e",
        );
        // test_composable.IVisualFactory (no methods)
        check(
            "test_composable.IVisualFactory:",
            "1974545d-259f-553c-8ea0-e505f897df81",
        );
    }

    #[test]
    fn guid_simple_method() {
        // test_component.Nested.IThing
        check(
            "test_component.Nested.IThing:HRESULT Method();",
            "5448be22-9873-5ae6-9106-f6e8455d2fdd",
        );
        // test_activation.One.IMissing
        check(
            "test_activation.One.IMissing:HRESULT Method();",
            "ad54a92f-16de-537c-b6c0-5099534ee12e",
        );
    }

    #[test]
    fn guid_property_getter() {
        // test_activation.One.IInstance: Int32 Property { get; }
        check(
            "test_activation.One.IInstance:HRESULT get_Property(Int32*);",
            "4cc554b9-8483-54a9-8490-1467dfd7078f",
        );
    }

    #[test]
    fn guid_composable_factory() {
        // test_constructors.IComposableFactory
        check(
            "test_constructors.IComposableFactory:HRESULT CreateInstance(Object*,Object**,test_constructors.Composable**);HRESULT WithValue(Int32,Object*,Object**,test_constructors.Composable**);",
            "6a461099-83c0-5810-9e20-2e8b9521d143",
        );
    }

    #[test]
    fn guid_generic_collection() {
        // Test.ITest (collection_interop): arrays expand to (UInt32, TypeName*), generic with space
        check(
            "Test.ITest:HRESULT TestIterable(Windows.Foundation.Collections.IIterable`1<Int32>*,UInt32,Int32*);HRESULT GetIterable(UInt32,Int32*,Windows.Foundation.Collections.IIterable`1<Int32>**);HRESULT GetMapView(UInt32,Int32*,Windows.Foundation.Collections.IMapView`2<Int32, Windows.Foundation.Collections.IVectorView`1<Int32>>**);",
            "ab9ee103-2921-5ff1-95b3-6b72ea1d289f",
        );
    }

    #[test]
    fn guid_composable_interfaces() {
        check(
            "test_composable.ICompositor:HRESULT CreateSpriteVisual(Int32,test_composable.SpriteVisual**);HRESULT CreateContainerVisual(Int32,test_composable.ContainerVisual**);",
            "ac7b49b8-e092-52ad-8456-48696a5a258e",
        );
        check(
            "test_composable.IVisual:HRESULT get_Compositor(test_composable.Compositor**);",
            "ce89606a-5b03-5861-af26-9dced3aab7e6",
        );
        check(
            "test_composable.IContainerVisual:HRESULT get_Children(Int32*);",
            "b8accc46-3ff7-5a24-8247-f5a52e1f5a8d",
        );
        check(
            "test_composable.ISpriteVisual:HRESULT get_Brush(Int32*);",
            "25f23ebe-4cd3-5349-b16d-d88c4d852ea1",
        );
    }

    #[test]
    fn guid_overloads() {
        // test_overloads.IA: Method() -> Int32, Method(Int32 a) -> Int32
        // midlrt auto-renames the second overload to "Method2"
        check(
            "test_overloads.IA:HRESULT Method(Int32*);HRESULT Method2(Int32,Int32*);",
            "ea3ed6f8-2f81-5cfc-a281-4bf0d7535521",
        );
    }
}

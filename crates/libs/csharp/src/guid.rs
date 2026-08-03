//! Generation-time computation of parameterized (generic) WinRT interface IIDs.
//!
//! A generic interface instantiation such as `IVector<Int32>` does not carry a `GuidAttribute`; its
//! IID is derived from the open generic's PIID and the type arguments per the WinRT rules (an
//! RFC-4122 v5 GUID: SHA1 over the WinRT parameterized-type namespace GUID followed by a
//! `pinterface({piid};<arg-signatures>)` signature string). windows-rs computes the same value at
//! compile time through a `const fn`; because an instantiation is concrete in the winmd, this
//! generator computes it once at generation time and emits it as a plain `new Guid(...)` literal,
//! so the projected C# pays nothing at run time.

use crate::model::Guid;
use windows_metadata::HasAttributes;
use windows_metadata::Type;
use windows_metadata::reader::{Index, TypeCategory, TypeDef};

/// The WinRT parameterized-type namespace GUID (`11f47ad5-7b73-42c0-abae-878b1e16adee`), prepended
/// to a type signature before hashing.
const NAMESPACE: [u8; 16] = [
    0x11, 0xf4, 0x7a, 0xd5, 0x7b, 0x73, 0x42, 0xc0, 0xab, 0xae, 0x87, 0x8b, 0x1e, 0x16, 0xad, 0xee,
];

impl Guid {
    /// Formats the GUID as a lowercase, brace-wrapped signature token (`{xxxxxxxx-....}`), the form
    /// a non-generic interface contributes to a parameterized signature.
    fn to_signature(self) -> String {
        format!(
            "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10
        )
    }

    /// Derives a parameterized IID from a WinRT type signature (an RFC-4122 v5 GUID over
    /// [`NAMESPACE`] + `signature`).
    pub fn from_signature(signature: &[u8]) -> Self {
        let mut data = Vec::with_capacity(NAMESPACE.len() + signature.len());
        data.extend_from_slice(&NAMESPACE);
        data.extend_from_slice(signature);
        let b = sha1(&data);

        let data1 = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let data2 = u16::from_be_bytes([b[4], b[5]]);
        let data3 = (u16::from_be_bytes([b[6], b[7]]) & 0x0fff) | (5 << 12);
        let data4_0 = (b[8] & 0x3f) | 0x80;
        Self(
            data1, data2, data3, data4_0, b[9], b[10], b[11], b[12], b[13], b[14], b[15],
        )
    }
}

/// Computes the IID of a generic interface instantiation from the open generic's PIID and the WinRT
/// signatures of its type arguments, or `None` if any argument has no representable signature.
pub fn generic_iid(index: &Index, open_piid: Guid, args: &[Type]) -> Option<Guid> {
    let signature = generic_signature(index, open_piid, args)?;
    Some(Guid::from_signature(signature.as_bytes()))
}

fn generic_signature(index: &Index, open_piid: Guid, args: &[Type]) -> Option<String> {
    let mut signature = format!("pinterface({}", open_piid.to_signature());
    for arg in args {
        signature.push(';');
        signature.push_str(&type_signature(index, arg)?);
    }
    signature.push(')');
    Some(signature)
}

/// Returns the WinRT type signature string for a type, or `None` for a shape whose signature the
/// generator does not compute. Used only to derive parameterized IIDs, so it need cover just the
/// element and argument types that appear inside a supported generic instantiation.
pub fn type_signature(index: &Index, ty: &Type) -> Option<String> {
    Some(match ty {
        Type::Bool => "b1".to_string(),
        Type::Char => "c2".to_string(),
        Type::I8 => "i1".to_string(),
        Type::U8 => "u1".to_string(),
        Type::I16 => "i2".to_string(),
        Type::U16 => "u2".to_string(),
        Type::I32 => "i4".to_string(),
        Type::U32 => "u4".to_string(),
        Type::I64 => "i8".to_string(),
        Type::U64 => "u8".to_string(),
        Type::F32 => "f4".to_string(),
        Type::F64 => "f8".to_string(),
        Type::String => "string".to_string(),
        Type::Object => "cinterface(IInspectable)".to_string(),
        Type::ValueName(tn) if tn.namespace == "System" && tn.name == "Guid" => "g16".to_string(),
        Type::ValueName(tn) => {
            let def = index.get(&tn.namespace, &tn.name).next()?;
            let name = format!("{}.{}", tn.namespace, tn.name);
            match def.category() {
                TypeCategory::Enum => {
                    let flags = if def.has_attribute("FlagsAttribute") {
                        "u4"
                    } else {
                        "i4"
                    };
                    format!("enum({name};{flags})")
                }
                TypeCategory::Struct => {
                    let mut fields = Vec::new();
                    for field in def.fields() {
                        fields.push(type_signature(index, &field.ty())?);
                    }
                    format!("struct({name};{})", fields.join(";"))
                }
                _ => return None,
            }
        }
        Type::ClassName(tn) if tn.generics.is_empty() => {
            let def = index.get(&tn.namespace, &tn.name).next()?;
            match def.category() {
                TypeCategory::Interface => guid_of(def)?.to_signature(),
                TypeCategory::Delegate => format!("delegate({})", guid_of(def)?.to_signature()),
                TypeCategory::Class => {
                    format!(
                        "rc({}.{};{})",
                        tn.namespace,
                        tn.name,
                        crate::default_interface_iid(index, def)?.to_signature()
                    )
                }
                _ => return None,
            }
        }
        Type::ClassName(tn) => {
            // A nested generic instantiation contributes its own parameterized IID. The Index keys
            // types by their arity-trimmed name (`IVector`, not `IVector`1`).
            let def = index
                .get(&tn.namespace, windows_metadata::trim_tick(&tn.name))
                .next()?;
            generic_signature(index, guid_of(def)?, &tn.generics)?
        }
        _ => return None,
    })
}

/// Reads a `GuidAttribute` into a [`Guid`], mirroring the reader in `lib.rs` (kept here so the
/// signature code is self-contained).
fn guid_of(def: TypeDef) -> Option<Guid> {
    crate::guid_attribute(def)
}

/// A standard (non-const) SHA1 over `data`, returning the 20-byte digest. WinRT parameterized IIDs
/// are v5 GUIDs, which are SHA1-based.
fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for (i, word) in w.iter_mut().take(16).enumerate() {
            *word = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let [mut a, mut b, mut c, mut d, mut e] = h;
        for (i, &wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let tmp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = tmp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut out = [0u8; 20];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivector_i32_iid_matches_windows_rs() {
        // Open IVector`1 PIID + Int32 argument. Ground truth from
        // `<windows_collections::IVector<i32> as windows_core::Interface>::IID`.
        let piid = Guid(
            0x913337e9, 0x11a1, 0x4345, 0xa3, 0xa2, 0x4e, 0x7f, 0x95, 0x6e, 0x22, 0x2d,
        );
        let signature = format!("pinterface({};i4)", piid.to_signature());
        let iid = Guid::from_signature(signature.as_bytes());
        assert_eq!(
            iid.to_cs(),
            Guid(
                0xB939AF5B, 0xB45D, 0x5489, 0x91, 0x49, 0x61, 0x44, 0x2c, 0x19, 0x05, 0xfe
            )
            .to_cs()
        );
    }

    #[test]
    fn imap_i32_i32_iid_matches_windows_rs() {
        // Open IMap`2 PIID + two Int32 arguments. Ground truth from
        // `<windows_collections::IMap<i32, i32> as windows_core::Interface>::IID`.
        let piid = Guid(
            0x3c2925fe, 0x8519, 0x45c1, 0xaa, 0x79, 0x19, 0x7b, 0x67, 0x18, 0xc1, 0xc1,
        );
        let signature = format!("pinterface({};i4;i4)", piid.to_signature());
        let iid = Guid::from_signature(signature.as_bytes());
        assert_eq!(
            iid.to_cs(),
            Guid(
                0x19da7f0f, 0xdb46, 0x5b15, 0x8e, 0x00, 0x27, 0xcb, 0xa1, 0xf7, 0xb4, 0x1d
            )
            .to_cs()
        );
    }

    #[test]
    fn ivectorview_i32_iid_matches_windows_rs() {
        // Open IVectorView`1 PIID + Int32 argument. Ground truth from
        // `<windows_collections::IVectorView<i32> as windows_core::Interface>::IID`.
        let piid = Guid(
            0xbbe1fa4c, 0xb0e3, 0x4583, 0xba, 0xef, 0x1f, 0x1b, 0x2e, 0x48, 0x3e, 0x56,
        );
        let signature = format!("pinterface({};i4)", piid.to_signature());
        let iid = Guid::from_signature(signature.as_bytes());
        assert_eq!(
            iid.to_cs(),
            Guid(
                0x8d720cdf, 0x3934, 0x5d3f, 0x9a, 0x55, 0x40, 0xe8, 0x06, 0x3b, 0x08, 0x6a
            )
            .to_cs()
        );
    }

    #[test]
    fn imapview_i32_i32_iid_matches_windows_rs() {
        // Open IMapView`2 PIID + two Int32 arguments. Ground truth from
        // `<windows_collections::IMapView<i32, i32> as windows_core::Interface>::IID`.
        let piid = Guid(
            0xe480ce40, 0xa338, 0x4ada, 0xad, 0xcf, 0x27, 0x22, 0x72, 0xe4, 0x8c, 0xb9,
        );
        let signature = format!("pinterface({};i4;i4)", piid.to_signature());
        let iid = Guid::from_signature(signature.as_bytes());
        assert_eq!(
            iid.to_cs(),
            Guid(
                0x14815e90, 0x9809, 0x56af, 0x9d, 0xc0, 0x74, 0x59, 0xa8, 0xf7, 0x28, 0x41
            )
            .to_cs()
        );
    }
}

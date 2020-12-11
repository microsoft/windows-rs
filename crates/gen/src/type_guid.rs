use crate::*;
use squote::{quote, Literal, TokenStream};

#[derive(Clone, Default, PartialEq)]
pub struct TypeGuid(pub [GuidConstant; 11]);

#[derive(Clone, PartialEq)]
pub enum GuidConstant {
    U32(u32),
    U16(u16),
    U8(u8),
}

impl std::fmt::Debug for GuidConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U32(value) => write!(f, "{:08x?}", value),
            Self::U16(value) => write!(f, "{:04x?}", value),
            Self::U8(value) => write!(f, "{:02x?}", value),
        }
    }
}

impl GuidConstant {
    fn from_arg(arg: &winmd::AttributeArg) -> GuidConstant {
        match *arg {
            winmd::AttributeArg::U32(value) => GuidConstant::U32(value),
            winmd::AttributeArg::U16(value) => GuidConstant::U16(value),
            winmd::AttributeArg::U8(value) => GuidConstant::U8(value),
            _ => panic!("GuidConstant.from_arg"),
        }
    }
}

impl std::fmt::Debug for TypeGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}-{:?}-{:?}-{:?}{:?}-{:?}{:?}{:?}{:?}{:?}{:?}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10]
        )
    }
}

impl Default for GuidConstant {
    fn default() -> Self {
        Self::U8(0)
    }
}

impl TypeGuid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_type_def(def: &winmd::TypeDef) -> Self {
        for attribute in def.attributes() {
            match attribute.name() {
                ("Windows.Foundation.Metadata", "GuidAttribute") => {
                    let args = attribute.args();

                    return Self([
                        GuidConstant::from_arg(&args[0].1),
                        GuidConstant::from_arg(&args[1].1),
                        GuidConstant::from_arg(&args[2].1),
                        GuidConstant::from_arg(&args[3].1),
                        GuidConstant::from_arg(&args[4].1),
                        GuidConstant::from_arg(&args[5].1),
                        GuidConstant::from_arg(&args[6].1),
                        GuidConstant::from_arg(&args[7].1),
                        GuidConstant::from_arg(&args[8].1),
                        GuidConstant::from_arg(&args[9].1),
                        GuidConstant::from_arg(&args[10].1),
                    ]);
                }
                ("System.Runtime.InteropServices", "GuidAttribute") => {
                    let args = attribute.args();

                    if let winmd::AttributeArg::String(guid) = &args[0].1 {
                        let guid = GuidAttribute::new(&guid);

                        return Self([
                            GuidConstant::U32(guid.a),
                            GuidConstant::U16(guid.b),
                            GuidConstant::U16(guid.c),
                            GuidConstant::U8(guid.d),
                            GuidConstant::U8(guid.e),
                            GuidConstant::U8(guid.f),
                            GuidConstant::U8(guid.g),
                            GuidConstant::U8(guid.h),
                            GuidConstant::U8(guid.i),
                            GuidConstant::U8(guid.j),
                            GuidConstant::U8(guid.k),
                        ]);
                    }
                }
                _ => {}
            }
        }

        return Self([
            GuidConstant::U32(0),
            GuidConstant::U16(0),
            GuidConstant::U16(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
            GuidConstant::U8(0),
        ]);
    }

    pub fn gen(&self) -> TokenStream {
        let mut iter = self.0.iter().map(|value| match value {
            GuidConstant::U32(value) => Literal::u32_unsuffixed(*value),
            GuidConstant::U16(value) => Literal::u16_unsuffixed(*value),
            GuidConstant::U8(value) => Literal::u8_unsuffixed(*value),
        });

        let three = iter.by_ref().take(3);

        quote! {
            #(#three,)* [#(#iter),*],
        }
    }
}

struct GuidAttribute {
    a: u32,
    b: u16,
    c: u16,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
    i: u8,
    j: u8,
    k: u8,
}

impl GuidAttribute {
    fn new(value: &str) -> Self {
        assert!(value.len() == 36, "Invalid GUID string");
        let mut bytes = value.bytes();

        let a = ((bytes.next_u32() * 16 + bytes.next_u32()) << 24)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 16)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 8)
            + bytes.next_u32() * 16
            + bytes.next_u32();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let b = ((bytes.next_u16() * 16 + (bytes.next_u16())) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let c = ((bytes.next_u16() * 16 + bytes.next_u16()) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let d = bytes.next_u8() * 16 + bytes.next_u8();
        let e = bytes.next_u8() * 16 + bytes.next_u8();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let f = bytes.next_u8() * 16 + bytes.next_u8();
        let g = bytes.next_u8() * 16 + bytes.next_u8();
        let h = bytes.next_u8() * 16 + bytes.next_u8();
        let i = bytes.next_u8() * 16 + bytes.next_u8();
        let j = bytes.next_u8() * 16 + bytes.next_u8();
        let k = bytes.next_u8() * 16 + bytes.next_u8();

        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
            k,
        }
    }
}

use super::*;

#[derive(Clone, PartialEq, Default)]
pub struct Guid(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8);

impl Guid {
    pub fn from_type_def(def: &tables::TypeDef) -> Option<Self> {
        for attribute in def.attributes() {
            match attribute.full_name() {
                ("Windows.Foundation.Metadata", "GuidAttribute")
                | ("Windows.Win32.Interop", "GuidAttribute") => {
                    let args = attribute.args();

                    return Some(Self(
                        args[0].1.unwrap_u32(),
                        args[1].1.unwrap_u16(),
                        args[2].1.unwrap_u16(),
                        args[3].1.unwrap_u8(),
                        args[4].1.unwrap_u8(),
                        args[5].1.unwrap_u8(),
                        args[6].1.unwrap_u8(),
                        args[7].1.unwrap_u8(),
                        args[8].1.unwrap_u8(),
                        args[9].1.unwrap_u8(),
                        args[10].1.unwrap_u8(),
                    ));
                }
                _ => {}
            }
        }

        None
    }

    pub fn gen(&self) -> TokenStream {
        let a = Literal::u32_unsuffixed(self.0);
        let b = Literal::u16_unsuffixed(self.1);
        let c = Literal::u16_unsuffixed(self.2);
        let d = Literal::u8_unsuffixed(self.3);
        let e = Literal::u8_unsuffixed(self.4);
        let f = Literal::u8_unsuffixed(self.5);
        let g = Literal::u8_unsuffixed(self.6);
        let h = Literal::u8_unsuffixed(self.7);
        let i = Literal::u8_unsuffixed(self.8);
        let j = Literal::u8_unsuffixed(self.9);
        let k = Literal::u8_unsuffixed(self.10);

        quote! {
            #a, #b, #c, [#d, #e, #f, #g, #h, #i, #j, #k],
        }
    }
}

impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
        )
    }
}

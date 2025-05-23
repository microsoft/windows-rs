use super::*;
use windows_metadata::reader::*;

impl Format for Item<'_> {
    fn format(&self, f: &mut Formatter) {
        match self {
            Item::Type(ty) => ty.format(f),
            _ => {}
        }
    }
}

impl Format for TypeDef<'_> {
    fn format(&self, f: &mut Formatter) {
        match self.category() {
            TypeCategory::Struct => {
                writeln!(f, "struct {} {{", self.name()).unwrap();
                f.indent();

                for field in self.fields() {
                    writeln!(f, "{}: {},", field.name(), field.ty()).unwrap();
                }

                f.dedent();
                writeln!(f, "}}").unwrap();
            }
            TypeCategory::Enum => {
                writeln!(f, "enum {} {{", self.name()).unwrap();
                f.indent();

                for field in self.fields() {
                    if let Some(constant) = field.constant() {
                        writeln!(f, "{} = {},", field.name(), constant.value()).unwrap();
                    }
                }

                f.dedent();
                writeln!(f, "}}").unwrap();
            }
            TypeCategory::Interface => {
                writeln!(f, "interface {} {{", trim_tick(self.name())).unwrap();
                f.indent();

                for method in self.methods() {
                    writeln!(f, "fn {}();", method.name()).unwrap();
                }

                f.dedent();
                writeln!(f, "}}").unwrap();
            }
            TypeCategory::Class => {
                writeln!(f, "class {} {{}}", self.name()).unwrap();
            }
            _ => {}
        }
    }
}

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}

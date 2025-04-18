use super::*;
use windows_ecma335::reader::*;

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
            // TypeCategory::Enum => {
            //     writeln!(f, "enum {} {{", self.name())?;

            //     for field in self.fields() {
            //         if let Some(constant) = field.constant() {
            //             writeln!(f, "    {} = {},", field.name(), constant.value())?;
            //         }
            //     }

            //     writeln!(f, "}}")
            // }
            // TypeCategory::Interface => {
            //     writeln!(f, "interface {} {{", trim_tick(self.name()))?;

            //     for method in self.methods() {
            //         writeln!(f, "    fn {}();", method.name())?;
            //     }

            //     writeln!(f, "}}")
            // }
            // TypeCategory::Class => {
            //     writeln!(f, "class {} {{}}", self.name())
            // }
            _ => {}
        }
    }
}

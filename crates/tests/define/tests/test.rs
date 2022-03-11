#[test]
fn define() {
    windows::core::define! {
        mod Microsoft {
            mod Windows {
                struct StructType { x: i32, y: i32 }

                interface IInterfaceType {
                    fn Method(&self) -> StructType;
                }

                class ClassType : IInterfaceType;
            }
        }
    }
}

#[test]
fn metagen() {
    windows_metagen::test();
}

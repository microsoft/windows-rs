use windows_ecma335::*;
mod printer;
use printer::*;

// TODO: 1. tool that generates standalone *.rdl files from *.winmd files describing APIs like IDL
//       2. tool that parses standalone *.rdl files and produces *.winmd files
//
// that way we can separate the problem of parsing header/idl files from the issue of bootstrapping a textual metadata format

// Something like https://github.com/microsoft/windows-rs/issues/1093#issuecomment-1065621437 could be used to take the place
// of the interface macro with different variants of the "define" macro to either spit out a winmd file or just directly generate
// Rust code like the interface macro does. But that way we have a single way to define APIs in Rust.

// TODO: Do like so with attributes so we can still run rustfmt on the code with pre/post processing
// fn main() {
//     define! {
//         mod Microsoft {
//             mod Windows {
//                 struct StructType { x: i32, y: i32 }

//                 // replace interface with trait and place a #[meta::interface] attribute here
//                 interface IInterfaceType {
//                     fn Method(&self) -> StructType;
//                 }

//                 // replace class with struct and place a #[meta::class] attribute here
//                 class ClassType(IInterfaceType, IStringable);
//             }
//         }
//     }
// }

// TODO: also support Cargo style modules in separate files e.g. `mod Windows;`

fn main() {
    let time = std::time::Instant::now();

    let index = reader::Index::read("crates/libs/bindgen/default/Windows.winmd").unwrap();
    let index = reader::MemberIndex::new(&index);

    let mut printer = Printer::new();

    for (namespace, types) in &*index {
        if !namespace.starts_with("Windows.Foundation") {
            continue;
        }

    for ty in types.values().flatten() {
            if let reader::Member::Type(ty) = ty {
                printer.write_namespace(ty.namespace());

                code!(printer, "{ty}");
            }
        }
    }

    std::fs::write("rdl.rs", printer.into_string()).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

// fn trim_tick(name: &str) -> &str {
//     if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
//         &name[..name.len() - 2]
//     } else {
//         name
//     }
// }

// TODO: maybe we use a custom prettyplease/fmt that knows how to deal with the keywords directly

// #[allow(dead_code, non_snake_case)]
// mod syntax {

//     trait IUnknown {
//         fn AddRef(&self);
//     }

//     trait IInspectable: IUnknown {
//         fn GetRuntimeClassName(&self);
//     }

//     trait IAsyncInfo: IInspectable {
//         fn Status(&self) -> AsyncStatus;
//     }

//     // This implies that IAsyncAction's vtable extends IInspectable's vtable as it appears first in the last while IAsyncAction "requires" IAsyncInfo.
//     trait IAsyncAction: IInspectable + IAsyncInfo {
//         fn GetResults(&self);
//     }

//     enum AsyncStatus {
//         Canceld = 2,
//     }

//     struct Size {
//         Width: f32,
//         Height: f32,
//     }

//     extern "C" {
//         fn AsyncActionCompletedHandler(sender: dyn IAsyncAction, status: AsyncStatus);
//     }

//     trait IDeferral: IInspectable {}
//     trait IClosable: IInspectable {}

//     /// #[class]
//     /// #[Activatable(IDeferralFactory)]
//     trait Deferral: IDeferral + IClosable {}
// }

// fn main() {
//     define! {
//         mod Microsoft {
//             mod Windows {
//                 struct StructType { x: i32, y: i32 }

//                 interface IInterfaceType {
//                     fn Method(&self) -> StructType;
//                 }

//                 class ClassType : IInterfaceType;
//             }
//         }
//     }
// }

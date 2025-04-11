use windows_ecma335::*;

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

fn  main() {


}

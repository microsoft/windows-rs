mod blobs;
mod gen;
mod helpers;
mod pe;
mod strings;
mod tables;
use blobs::*;

use super::*;
pub use gen::*;
use helpers::*;
use std::collections::*;
use strings::*;
use tables::*;

pub fn test() {
    let mut tables = Tables::new();
    tables.module.push(Module::new("test.winmd"));
    tables.type_def.push(TypeDef::module());

    let mut stringable = TypeDef::winrt_interface("IStringable", "Windows.Foundation");
    stringable.method_list.push(MethodDef::new("ToString"));
    tables.type_def.push(stringable);

    let mut closable = TypeDef::winrt_interface("IClosable", "Windows.Foundation");
    closable.method_list.push(MethodDef::new("Close"));
    tables.type_def.push(closable);

    pe::write("/git/test.winmd", tables);
}

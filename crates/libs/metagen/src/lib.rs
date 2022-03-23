// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

mod blobs;
mod pe;
mod strings;
mod tables;
use blobs::*;
use std::collections::*;
use std::mem::*;
use std::slice::*;
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

fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

trait Write {
    fn write<T: Sized>(&mut self, value: &T);
}

impl Write for Vec<u8> {
    fn write<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
        }
    }
}

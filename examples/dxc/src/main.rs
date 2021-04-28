use bindings::Windows::Win32::{
    Globalization::CP_UTF8, Graphics::Hlsl::*, System::SystemServices::BOOL,
};
use libloading::{Library, Symbol};
use std::path::Path;
use windows::*;

// Only exists in newer DXC headers
const DXC_CP_UTF8: u32 = CP_UTF8;

#[cfg(target_os = "windows")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("dxcompiler.dll")
}

#[cfg(target_os = "linux")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("./libdxcompiler.so")
}

#[cfg(target_os = "macos")]
fn dxcompiler_lib_name() -> &'static Path {
    Path::new("./libdxcompiler.dynlib")
}

fn blob_encoding_as_str(blob: &IDxcBlobEncoding) -> &str {
    let mut known: BOOL = false.into();
    let mut cp = 0;
    unsafe { blob.GetEncoding(known.set_abi(), cp.set_abi()) }.unwrap();
    assert!(bool::from(known));
    assert_eq!(cp, DXC_CP_UTF8);
    unsafe {
        let slice = std::slice::from_raw_parts(
            blob.GetBufferPointer() as *const u8,
            blob.GetBufferSize() - 1,
        );
        std::str::from_utf8_unchecked(slice)
    }
}

fn main() -> windows::Result<()> {
    let lib = unsafe { Library::new(dxcompiler_lib_name()) }.unwrap();
    let create: Symbol<DxcCreateInstanceProc> = unsafe { lib.get(b"DxcCreateInstance\0") }.unwrap();
    dbg!(&create);

    let compiler: IDxcCompiler2 = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcCompiler) }?;
    let library: IDxcLibrary = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcLibrary) }?;

    dbg!(&compiler, &library);

    let main_shader = include_str!("copy.hlsl");

    let mut blob = None;
    let shader_blob = unsafe {
        library.CreateBlobWithEncodingFromPinned(
            main_shader.as_ptr() as *const _,
            main_shader.len() as u32,
            DXC_CP_UTF8,
            &mut blob,
        )
    }
    .and_some(blob)?;
    dbg!(&shader_blob);

    let mut args = vec![];
    let defines = vec![];

    let mut result = None;
    let result = unsafe {
        compiler.Compile(
            shader_blob,
            "copy.hlsl",
            "copyCs",
            "cs_6_5",
            args.as_mut_ptr(), // Should not be mut?
            args.len() as u32,
            defines.as_ptr(),
            defines.len() as u32,
            None,
            &mut result,
        )
    }
    .and_some(result)?;

    let mut status = HRESULT::default();
    unsafe { result.GetStatus(&mut status) }.ok()?;
    if status.is_err() {
        let mut errors = None;
        let errors = unsafe { result.GetErrorBuffer(&mut errors) }.and_some(errors)?;
        let errors = blob_encoding_as_str(&errors);
        eprintln!("Compilation failed with {:?}: `{}`", status, errors);
        status.ok()
    } else {
        let mut blob = None;
        let blob = unsafe { result.GetResult(&mut blob) }.and_some(blob)?;
        let shader = unsafe {
            std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
        };
        for c in shader.chunks(16) {
            println!("{:02x?}", c);
        }
        Ok(())
    }
}

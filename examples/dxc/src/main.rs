use bindings::Windows::Win32::{
    Globalization::CP_UTF8,
    Graphics::Hlsl::*,
    System::{
        Diagnostics::Debug::ERROR_FILE_NOT_FOUND,
        SystemServices::{BOOL, PWSTR},
    },
};
use libloading::{Library, Symbol};
use std::path::Path;
use std::rc::Rc;
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

fn create_blob(library: &IDxcLibrary, data: &str) -> windows::Result<IDxcBlobEncoding> {
    let mut blob = None;
    unsafe {
        library.CreateBlobWithEncodingFromPinned(
            data.as_ptr() as *const _,
            data.len() as u32,
            DXC_CP_UTF8,
            &mut blob,
        )
    }
    .and_some(blob)
}

#[allow(non_snake_case)]
fn main() -> windows::Result<()> {
    let lib = unsafe { Library::new(dxcompiler_lib_name()) }.unwrap();
    let create: Symbol<DxcCreateInstanceProc> = unsafe { lib.get(b"DxcCreateInstance\0") }.unwrap();
    dbg!(&create);

    let compiler: IDxcCompiler2 = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcCompiler) }?;
    let library: IDxcLibrary = unsafe { DxcCreateInstanceProc(&create, &CLSID_DxcLibrary) }?;

    dbg!(&compiler, &library);

    let main_shader = "#include \"copy.hlsl\"";
    let shader_blob = create_blob(&library, main_shader)?;

    unsafe extern "system" fn LoadSource(
        this: RawPtr,
        pfilename: PWSTR,
        ppincludesource: *mut RawPtr,
    ) -> HRESULT {
        let this = &mut *(this as *mut IncludeHandlerData);
        dbg!(&this, pfilename, ppincludesource);

        if pfilename != "./foo/bar/copy.hlsl" {
            return HRESULT::from_win32(ERROR_FILE_NOT_FOUND.0);
        }
        let copy_shader = include_str!("copy.hlsl");
        let shader_blob = create_blob(&this.library, copy_shader).unwrap();
        *ppincludesource = shader_blob.abi();
        this.alive_shaders.push(shader_blob);
        HRESULT(0)
    }

    unsafe extern "system" fn QueryInterface(
        _ptr: RawPtr,
        _iid: &Guid,
        _interface: *mut RawPtr,
    ) -> HRESULT {
        todo!()
    }

    unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
        let this = &mut *(this as *mut IncludeHandlerData);
        dbg!(this.refs.add_ref())
    }

    unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
        let this = &mut *(this as *mut IncludeHandlerData);
        dbg!(this.refs.release())
    }

    #[cfg(not(windows))]
    unsafe extern "system" fn dtor(_ptr: ::windows::RawPtr) {
        todo!()
    }

    let include_handler = IDxcIncludeHandler_abi(
        // IUnknown
        QueryInterface,
        AddRef,
        Release,
        // Virtual destructors breaking the vtable, hack for !windows DXC
        #[cfg(not(windows))]
        dtor,
        #[cfg(not(windows))]
        dtor,
        LoadSource,
    );

    #[derive(Debug)]
    struct IncludeHandlerData {
        vtable: *const IDxcIncludeHandler_abi,
        refs: RefCount,
        library: Rc<IDxcLibrary>,
        alive_shaders: Vec<IDxcBlobEncoding>,
    }

    #[derive(Debug)]
    struct IncludeHandler(std::ptr::NonNull<IncludeHandlerData>);

    let library = Rc::new(library);

    let handler_data = Box::new(IncludeHandlerData {
        vtable: &include_handler,
        refs: RefCount::new(1),
        library,
        alive_shaders: vec![],
    });

    let handler =
        IncludeHandler(unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(handler_data)) });

    unsafe impl ::windows::Interface for IncludeHandler {
        type Vtable = IDxcIncludeHandler_abi;
        const IID: ::windows::Guid = ::windows::Guid::from_values(
            2137128061,
            38157,
            18047,
            [179, 227, 60, 2, 251, 73, 24, 124],
        );
    }
    impl ::std::convert::From<IncludeHandler> for IDxcIncludeHandler {
        fn from(value: IncludeHandler) -> Self {
            unsafe { ::std::mem::transmute(value) }
        }
    }
    impl ::std::convert::From<&IncludeHandler> for &IDxcIncludeHandler {
        fn from(value: &IncludeHandler) -> Self {
            unsafe { ::std::mem::transmute(value) }
        }
    }
    impl<'a> IntoParam<'a, IDxcIncludeHandler> for IncludeHandler {
        fn into_param(self) -> Param<'a, IDxcIncludeHandler> {
            Param::Owned(::std::convert::Into::<IDxcIncludeHandler>::into(self))
        }
    }
    impl<'a> IntoParam<'a, IDxcIncludeHandler> for &'a IncludeHandler {
        fn into_param(self) -> Param<'a, IDxcIncludeHandler> {
            Param::Borrowed(::std::convert::Into::<&IDxcIncludeHandler>::into(self))
        }
    }

    dbg!(&handler);

    let mut args = vec![];
    let defines = vec![];
    let mut result = None;
    let result = unsafe {
        compiler.Compile(
            shader_blob,
            "foo/bar/baz.hlsl",
            "copyCs",
            "cs_6_5",
            args.as_mut_ptr(), // Should not be mut?
            args.len() as u32,
            defines.as_ptr(),
            defines.len() as u32,
            // TODO: This also accepts a borrow which does not decrease our refcount to 0
            handler,
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

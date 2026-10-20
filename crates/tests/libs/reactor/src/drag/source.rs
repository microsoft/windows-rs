use super::*;
use std::cell::Cell;
use std::sync::atomic::AtomicU32;
use std::time::{Duration, Instant};
use windows_core::{BOOL, Error, Interface, Ref, Result};

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}
use bindings::*;

#[derive(Clone)]
pub struct Outcome {
    pub operation: Option<DragDropOperation>,
    pub text_read: bool,
    pub cancelled: bool,
}

struct Ole;

impl Drop for Ole {
    fn drop(&mut self) {
        unsafe { OleUninitialize() };
    }
}

pub fn run(
    completion: ComponentCompletion<Message>,
    cancelled: Arc<AtomicBool>,
    stopping: Arc<AtomicBool>,
    thread: Arc<AtomicU32>,
) -> Result<()> {
    unsafe { OleInitialize(std::ptr::null()).ok()? };
    let _ole = Ole;
    let window = SourceWindow::new()?;
    thread.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    if !completion.clone().complete(Message::Ready) {
        return Err(Error::new(E_ABORT, "drag fixture was removed"));
    }
    unsafe {
        let mut message = MSG::default();
        while !stopping.load(Ordering::Relaxed) {
            let status = GetMessageW(&mut message, std::ptr::null_mut(), 0, 0).0;
            if status == -1 {
                return Err(Error::from_thread());
            }
            if status == 0 {
                if !stopping.load(Ordering::Relaxed) {
                    return Err(Error::new(E_ABORT, "drag source stopped unexpectedly"));
                }
                break;
            }
            if message.hwnd == window.0 && message.message == WM_LBUTTONDOWN as u32 {
                let result = drag(completion.clone(), cancelled.clone());
                if !completion.clone().complete(Message::Finished(result)) {
                    return Err(Error::new(E_ABORT, "drag fixture was removed"));
                }
            } else {
                DispatchMessageW(&message);
            }
        }
    }
    window.close()
}

pub fn stop(thread: u32) -> Result<()> {
    if thread != 0 {
        unsafe { PostThreadMessageW(thread, WM_QUIT as u32, 0, 0).ok()? };
    }
    Ok(())
}

fn drag(completion: ComponentCompletion<Message>, cancelled: Arc<AtomicBool>) -> Result<Outcome> {
    let deadline = Instant::now() + Duration::from_secs(20);
    let reads = Arc::new(AtomicU32::new(0));
    let data: IDataObject = TextData {
        reads: reads.clone(),
    }
    .into();
    let source: IDropSource = Source {
        completion,
        cancelled,
        deadline,
        feedback: Cell::new(None),
    }
    .into();
    let mut effect = 0;
    let status = unsafe {
        DoDragDrop(
            data.as_raw(),
            source.as_raw(),
            (DROPEFFECT_COPY | DROPEFFECT_MOVE) as u32,
            &mut effect,
        )
    };
    if Instant::now() >= deadline {
        return Err(Error::new(E_ABORT, "drag source timed out"));
    }
    status.ok()?;
    if status != DRAGDROP_S_DROP && status != DRAGDROP_S_CANCEL {
        return Err(Error::new(
            E_UNEXPECTED,
            "unexpected drag completion status",
        ));
    }
    let cancelled = status == DRAGDROP_S_CANCEL;
    Ok(Outcome {
        operation: if cancelled { None } else { operation(effect)? },
        text_read: reads.load(Ordering::Relaxed) != 0,
        cancelled,
    })
}

fn operation(effect: u32) -> Result<Option<DragDropOperation>> {
    match effect {
        0 => Ok(None),
        value if value == DROPEFFECT_COPY as u32 => Ok(Some(DragDropOperation::Copy)),
        value if value == DROPEFFECT_MOVE as u32 => Ok(Some(DragDropOperation::Move)),
        _ => Err(Error::new(E_UNEXPECTED, "unexpected native drag operation")),
    }
}

#[windows_core::implement(IDropSource)]
struct Source {
    completion: ComponentCompletion<Message>,
    cancelled: Arc<AtomicBool>,
    deadline: Instant,
    feedback: Cell<Option<u32>>,
}

#[allow(non_snake_case)]
impl IDropSource_Impl for Source_Impl {
    fn QueryContinueDrag(&self, escape: BOOL, keys: u32) -> Result<()> {
        // OLE uses positive HRESULTs to cancel, drop, and select its default cursor.
        if escape.as_bool()
            || self.cancelled.load(Ordering::Relaxed)
            || Instant::now() >= self.deadline
        {
            Err(Error::from_hresult(DRAGDROP_S_CANCEL))
        } else if keys & MK_LBUTTON as u32 == 0 {
            Err(Error::from_hresult(DRAGDROP_S_DROP))
        } else {
            Ok(())
        }
    }

    fn GiveFeedback(&self, effect: u32) -> Result<()> {
        let effect = effect & !DROPEFFECT_SCROLL;
        let operation = operation(effect)?;
        if self.feedback.replace(Some(effect)) != Some(effect)
            && !self
                .completion
                .clone()
                .complete(Message::Feedback(operation))
        {
            return Err(Error::new(E_ABORT, "drag fixture was removed"));
        }

        Err(Error::from_hresult(DRAGDROP_S_USEDEFAULTCURSORS))
    }
}

struct SourceWindow(HWND);

impl SourceWindow {
    fn new() -> Result<Self> {
        unsafe {
            let mut point = POINT::default();
            GetCursorPos(&mut point).ok()?;
            let owner = GetForegroundWindow();
            let mut process = 0;
            GetWindowThreadProcessId(owner, &mut process);
            if process != GetCurrentProcessId() {
                return Err(Error::new(E_ABORT, "drag fixture is not foreground"));
            }
            let window = Self(CreateWindowExW(
                0,
                windows_core::w!("STATIC"),
                windows_core::w!("Drag text"),
                WS_POPUP | (WS_VISIBLE | WS_BORDER | SS_NOTIFY) as u32,
                point.x - 50,
                point.y - 25,
                100,
                50,
                owner,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            ));
            if window.0.is_null() {
                return Err(Error::from_thread());
            }
            Ok(window)
        }
    }

    fn close(mut self) -> Result<()> {
        let window = std::mem::replace(&mut self.0, std::ptr::null_mut());
        unsafe { DestroyWindow(window).ok() }
    }
}

impl Drop for SourceWindow {
    fn drop(&mut self) {
        if !self.0.is_null() && !unsafe { DestroyWindow(self.0) }.as_bool() {
            eprintln!("failed to destroy drag source: {}", Error::from_thread());
        }
    }
}

#[windows_core::implement(IDataObject)]
struct TextData {
    reads: Arc<AtomicU32>,
}

fn format() -> FORMATETC {
    FORMATETC {
        cfFormat: CF_UNICODETEXT as u16,
        dwAspect: DVASPECT_CONTENT as u32,
        lindex: -1,
        tymed: TYMED_HGLOBAL as u32,
        ..Default::default()
    }
}

#[allow(non_snake_case)]
impl IDataObject_Impl for TextData_Impl {
    fn GetData(&self, requested: *const FORMATETC) -> Result<STGMEDIUM> {
        self.QueryGetData(requested)?;
        let text: Vec<u16> = "Reactor native drag text"
            .encode_utf16()
            .chain([0])
            .collect();
        unsafe {
            let memory = GlobalMemory(GlobalAlloc(GMEM_MOVEABLE as u32, text.len() * 2));
            if memory.0.is_null() {
                return Err(Error::from_thread());
            }
            let data = GlobalLock(memory.0).cast::<u16>();
            if data.is_null() {
                return Err(Error::from_thread());
            }
            std::ptr::copy_nonoverlapping(text.as_ptr(), data, text.len());
            SetLastError(0);
            if !GlobalUnlock(memory.0).as_bool() && GetLastError() != 0 {
                return Err(Error::from_thread());
            }
            let medium = STGMEDIUM {
                tymed: TYMED_HGLOBAL as u32,
                Anonymous: uSTGMEDIUM_0 { hGlobal: memory.0 },
                ..Default::default()
            };
            std::mem::forget(memory);
            self.reads.fetch_add(1, Ordering::Relaxed);
            Ok(medium)
        }
    }

    fn QueryGetData(&self, requested: *const FORMATETC) -> Result<()> {
        let requested =
            unsafe { requested.as_ref() }.ok_or_else(|| Error::from_hresult(E_POINTER))?;
        if requested.cfFormat != CF_UNICODETEXT as u16
            || requested.dwAspect != DVASPECT_CONTENT as u32
            || requested.lindex != -1
            || requested.tymed & TYMED_HGLOBAL as u32 == 0
        {
            return Err(Error::from_hresult(DV_E_FORMATETC));
        }
        Ok(())
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, output: *mut FORMATETC) -> Result<()> {
        let output = unsafe { output.as_mut() }.ok_or_else(|| Error::from_hresult(E_POINTER))?;
        output.ptd = std::ptr::null_mut();
        Err(Error::from_hresult(DATA_S_SAMEFORMATETC))
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }

    fn EnumFormatEtc(&self, direction: u32) -> Result<IEnumFORMATETC> {
        if direction != DATADIR_GET as u32 {
            return Err(Error::from_hresult(E_NOTIMPL));
        }
        unsafe {
            let mut raw = std::ptr::null_mut();
            SHCreateStdEnumFmtEtc(1, &format(), &mut raw).ok()?;
            Ok(IEnumFORMATETC::from_raw(raw))
        }
    }

    fn DAdvise(&self, _: *const FORMATETC, _: u32, _: Ref<IAdviseSink>) -> Result<u32> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }
}

struct GlobalMemory(HGLOBAL);

impl Drop for GlobalMemory {
    fn drop(&mut self) {
        if !self.0.is_null() && !unsafe { GlobalFree(self.0) }.is_null() {
            eprintln!("failed to release drag data: {}", Error::from_thread());
        }
    }
}

#[test]
fn text_data_transfers_owned_unicode_memory() -> Result<()> {
    struct Medium(STGMEDIUM);
    impl Drop for Medium {
        fn drop(&mut self) {
            unsafe { ReleaseStgMedium(&mut self.0) };
        }
    }

    let reads = Arc::new(AtomicU32::new(0));
    let data: IDataObject = TextData {
        reads: reads.clone(),
    }
    .into();
    unsafe {
        assert_eq!(data.QueryGetData(std::ptr::null()), E_POINTER);
        for invalid in [
            FORMATETC {
                cfFormat: 0,
                ..format()
            },
            FORMATETC {
                dwAspect: 0,
                ..format()
            },
            FORMATETC {
                lindex: 0,
                ..format()
            },
            FORMATETC {
                tymed: 0,
                ..format()
            },
        ] {
            assert_eq!(data.QueryGetData(&invalid), DV_E_FORMATETC);
            assert_eq!(data.GetData(&invalid).err().unwrap().code(), DV_E_FORMATETC);
        }
        assert_eq!(reads.load(Ordering::Relaxed), 0);
        data.QueryGetData(&format()).ok()?;
        let first = Medium(data.GetData(&format())?);
        let second = Medium(data.GetData(&format())?);
        assert_ne!(first.0.Anonymous.hGlobal, second.0.Anonymous.hGlobal);
        assert_eq!(reads.load(Ordering::Relaxed), 2);
        drop(data);

        let expected: Vec<u16> = "Reactor native drag text"
            .encode_utf16()
            .chain([0])
            .collect();
        for medium in [first, second] {
            assert_eq!(medium.0.tymed, TYMED_HGLOBAL as u32);
            assert!(medium.0.pUnkForRelease.is_none());
            let memory = medium.0.Anonymous.hGlobal;
            assert!(GlobalSize(memory) >= expected.len() * 2);
            let pointer = GlobalLock(memory).cast::<u16>();
            assert!(!pointer.is_null());
            let text = std::slice::from_raw_parts(pointer, expected.len()).to_vec();
            SetLastError(0);
            let unlocked = GlobalUnlock(memory).as_bool() || GetLastError() == 0;
            assert!(unlocked);
            assert_eq!(text, expected);
        }
    }
    Ok(())
}

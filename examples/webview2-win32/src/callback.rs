use std::{marker::PhantomData, sync::mpsc};

use windows::{Abi, Interface};

use bindings::{
    Microsoft::Web::WebView2::Win32 as WebView2,
    Windows::Win32::SystemServices::{E_NOINTERFACE, E_POINTER, PWSTR, S_OK},
};

use super::{pwstr::string_from_pwstr, wait_with_pump};

/// Common trait for [`CompletedCallback`] and [`EventCallback`].
pub trait Callback<'a> {
    /// [`windows::Interface`] implemented by this [`Callback`].
    type Interface: 'a + Interface;
    /// Boxed closure type, either [`CompletedClosure`] or [`EventClosure`].
    type Closure: 'a;
}

/// Common trait for [`CompletedCallback`] and [`EventCallback`] which uses default method
/// implementations to support the [`windows::IUnknown`] interface.
#[allow(non_snake_case)]
pub trait CallbackInterface<'a, T: Callback<'a>>: Sized {
    fn ref_count(&self) -> &windows::RefCount;

    unsafe extern "system" fn QueryInterface(
        this: windows::RawPtr,
        iid: &windows::Guid,
        interface: *mut windows::RawPtr,
    ) -> windows::HRESULT {
        if interface.is_null() {
            E_POINTER
        } else if *iid == windows::IUnknown::IID
            || *iid == <<T as Callback>::Interface as Interface>::IID
        {
            Self::AddRef(this);
            *interface = this;
            S_OK
        } else {
            E_NOINTERFACE
        }
    }

    unsafe extern "system" fn AddRef(this: windows::RawPtr) -> u32 {
        let interface = this as *mut Self;
        (*interface).ref_count().add_ref()
    }

    unsafe extern "system" fn Release(this: windows::RawPtr) -> u32 {
        let interface = this as *mut Self;
        let count = (*interface).ref_count().release();
        if count == 0 {
            // Destroy the underlying data
            Box::from_raw(interface);
        }
        count
    }
}

/// Argument conversion trait, which translates from [`bindings::Windows::Win32`] types passed
/// to the [`CompletedCallback::Invoke`] or [`EventCallback::Invoke`] COM method to the `Rust`
/// types passed to the [`Callback::Closure`].
pub trait ClosureArg<'a> {
    type Input: 'a;
    type Output: 'a;

    /// Convert from the [`ClosureArg::Input`] type to [`ClosureArg::Output`]
    fn convert(input: Self::Input) -> Self::Output;
}

/// Pass-through argument conversion for [`windows::HRESULT`].
pub struct ErrorCodeArg;

impl<'a> ClosureArg<'a> for ErrorCodeArg {
    type Input = windows::HRESULT;
    type Output = windows::HRESULT;

    /// Pass-through argument conversion for [`windows::HRESULT`].
    fn convert(input: windows::HRESULT) -> windows::HRESULT {
        input
    }
}

/// Argument conversion from [`windows::RawPtr`] to [`windows::Interface`].
pub struct InterfaceArg<I: Interface>(PhantomData<I>);

impl<'a, I: 'a + Interface> ClosureArg<'a> for InterfaceArg<I> {
    type Input = windows::RawPtr;
    type Output = Option<I>;

    /// Convert from [`windows::RawPtr`] to an owned [`windows::Interface`].
    fn convert(input: windows::RawPtr) -> Option<I> {
        if input.is_null() {
            None
        } else {
            match unsafe { Self::from_abi(input) } {
                Ok(interface) => Some(interface),
                Err(_) => None,
            }
        }
    }
}

impl<'a, I: 'a + Interface> InterfaceArg<I> {
    /// Helper method to wrap [`windows::RawPtr`] in [`Interface`].
    unsafe fn from_abi(this: windows::RawPtr) -> windows::Result<I> {
        let unknown = windows::IUnknown::from_abi(this)?;
        unknown.vtable().1(unknown.abi()); // add_ref to balance the release called in IUnknown::drop
        unknown.cast()
    }
}

/// Argument conversion from [`PWSTR`] to [`String`], without requiring the [`PWSTR`] to be
/// allocated by `Rust`.
pub struct StringArg;

impl<'a> ClosureArg<'a> for StringArg {
    type Input = PWSTR;
    type Output = String;

    /// Convert from [`PWSTR`] to an owned [`String`] without requiring the [`PWSTR`] to be
    /// allocated by `Rust`.
    fn convert(input: PWSTR) -> String {
        string_from_pwstr(input)
    }
}

/// Generic closure signature for [`CompletedCallback`].
pub type CompletedClosure<'a, Arg1, Arg2> = Box<
    dyn 'a
        + FnOnce(
            <Arg1 as ClosureArg<'a>>::Output,
            <Arg2 as ClosureArg<'a>>::Output,
        ) -> windows::HRESULT,
>;

/// All of the async operations in [`WebView2`] use a completed handler interface which implements
/// a COM method matching [`CompletedCallback::Invoke`]. The [`CompletedCallback::Arg1::Input`] and
/// [`CompletedCallback::Arg2::Input`] parameter types vary depending on the async operation. Each
/// instance of the completed handler can only execute once.
#[allow(non_snake_case)]
pub trait CompletedCallback<'a, T, Arg1, Arg2>: CallbackInterface<'a, T>
where
    T: Callback<'a>,
    Arg1: ClosureArg<'a>,
    Arg2: ClosureArg<'a>,
{
    fn completed(&mut self) -> Option<CompletedClosure<'a, Arg1, Arg2>>;

    unsafe extern "system" fn Invoke(
        this: windows::RawPtr,
        arg_1: Arg1::Input,
        arg_2: Arg2::Input,
    ) -> windows::HRESULT {
        let interface = this as *mut Self;
        match (*interface).completed() {
            Some(completed) => completed(Arg1::convert(arg_1), Arg2::convert(arg_2)),
            None => S_OK,
        }
    }
}

/// Generic closure signature for [`EventCallback`].
pub type EventClosure<'a, Arg1, Arg2> = Box<
    dyn 'a
        + FnMut(
            <Arg1 as ClosureArg<'a>>::Output,
            <Arg2 as ClosureArg<'a>>::Output,
        ) -> windows::HRESULT,
>;

/// All of the async events in [`WebView2`] use a repeatable handler interface which implements
/// a COM method matching [`EventCallback::Invoke`]. The [`EventCallback::Arg1::Input`] and
/// [`EventCallback::Arg2::Input`] parameter types vary depending on the async operation.
#[allow(non_snake_case)]
pub trait EventCallback<'a, T, Arg1, Arg2>: CallbackInterface<'a, T>
where
    T: Callback<'a>,
    Arg1: ClosureArg<'a>,
    Arg2: ClosureArg<'a>,
{
    fn event(&mut self) -> &mut EventClosure<'a, Arg1, Arg2>;

    unsafe extern "system" fn Invoke(
        this: windows::RawPtr,
        arg_1: Arg1::Input,
        arg_2: Arg2::Input,
    ) -> windows::HRESULT {
        let interface = this as *mut Self;
        ((*interface).event())(Arg1::convert(arg_1), Arg2::convert(arg_2))
    }
}

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    WebView2::ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    ErrorCodeArg,
    InterfaceArg<WebView2::ICoreWebView2Environment>,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    WebView2::ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    ErrorCodeArg,
    InterfaceArg<WebView2::ICoreWebView2Controller>,
);

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    WebView2::ICoreWebView2WebMessageReceivedEventHandler,
    InterfaceArg<WebView2::ICoreWebView2>,
    InterfaceArg<WebView2::ICoreWebView2WebMessageReceivedEventArgs>,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    WebView2::ICoreWebView2WebResourceRequestedEventHandler,
    InterfaceArg<WebView2::ICoreWebView2>,
    InterfaceArg<WebView2::ICoreWebView2WebResourceRequestedEventArgs>,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    WebView2::ICoreWebView2PermissionRequestedEventHandler,
    InterfaceArg<WebView2::ICoreWebView2>,
    InterfaceArg<WebView2::ICoreWebView2PermissionRequestedEventArgs>,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    WebView2::ICoreWebView2NavigationCompletedEventHandler,
    InterfaceArg<WebView2::ICoreWebView2>,
    InterfaceArg<WebView2::ICoreWebView2NavigationCompletedEventArgs>,
);

#[completed_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    WebView2::ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    ErrorCodeArg,
    StringArg,
);

#[completed_callback]
pub struct ExecuteScriptCompletedHandler(
    WebView2::ICoreWebView2ExecuteScriptCompletedHandler,
    ErrorCodeArg,
    StringArg,
);

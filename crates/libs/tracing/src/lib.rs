#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![no_std]

use core::ffi::c_void;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};

#[expect(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings;
use bindings::*;

pub use windows_core::GUID;
pub use windows_result::{HRESULT, Result, WIN32_ERROR};
pub use windows_tracing_macros::{define_provider, write_event};

const ERROR_ALREADY_EXISTS: u32 = 183;

/// An ETW event level.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Level(pub u8);

impl Level {
    /// Events that are always emitted when the provider is enabled.
    pub const LOG_ALWAYS: Self = Self(0);

    /// Critical errors.
    pub const CRITICAL: Self = Self(1);

    /// Errors.
    pub const ERROR: Self = Self(2);

    /// Warnings.
    pub const WARNING: Self = Self(3);

    /// Informational events.
    pub const INFORMATIONAL: Self = Self(4);

    /// Verbose diagnostic events.
    pub const VERBOSE: Self = Self(5);
}

/// A static TraceLogging provider.
pub struct Provider {
    id: GUID,
    name: &'static str,
    metadata: &'static [u8],
    state: ProviderState,
}

impl Provider {
    /// Creates the provider representation used by [`define_provider`].
    #[doc(hidden)]
    pub const fn __new(id: GUID, name: &'static str, metadata: &'static [u8]) -> Self {
        Self {
            id,
            name,
            metadata,
            state: ProviderState::new(),
        }
    }

    /// Returns the provider identifier.
    pub const fn id(&self) -> &GUID {
        &self.id
    }

    /// Returns the provider name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the encoded provider metadata used by generated event macros.
    #[doc(hidden)]
    pub const fn __metadata(&self) -> &'static [u8] {
        self.metadata
    }

    /// Returns whether a trace session is listening for an event.
    #[inline(always)]
    pub fn enabled(&self, level: Level, keyword: u64) -> bool {
        self.state.enabled(level, keyword)
    }

    /// Registers this provider with ETW.
    ///
    /// The returned registration unregisters the provider when dropped.
    ///
    /// # Safety
    ///
    /// A registration created by a DLL must be dropped before the DLL unloads. ETW retains a
    /// callback into the module until the provider is unregistered.
    pub unsafe fn register(&'static self) -> Result<Registration> {
        let metadata_status = self.state.register(&self.id, self.metadata)?;
        Ok(Registration {
            provider: Some(self),
            metadata_status,
        })
    }

    #[doc(hidden)]
    pub fn __write(
        &self,
        descriptor: &EventDescriptor,
        data: &[EventDataDescriptor<'_>],
    ) -> WIN32_ERROR {
        self.state.write(descriptor, data)
    }
}

/// An active ETW provider registration.
#[must_use = "the provider unregisters when the registration is dropped"]
pub struct Registration {
    provider: Option<&'static Provider>,
    metadata_status: WIN32_ERROR,
}

impl core::fmt::Debug for Registration {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Registration")
            .field("provider", &self.provider.map(Provider::name))
            .field("metadata_status", &self.metadata_status)
            .finish()
    }
}

impl Registration {
    /// Returns the result from publishing the provider name and traits to ETW.
    ///
    /// Event writing remains available when this operation is unsupported because each event also
    /// carries the provider metadata required for decoding.
    pub const fn metadata_status(&self) -> WIN32_ERROR {
        self.metadata_status
    }

    /// Unregisters the provider and reports any ETW error.
    pub fn unregister(&mut self) -> Result<()> {
        let Some(provider) = self.provider else {
            return Ok(());
        };
        let status = provider.state.unregister();
        if status.is_ok() {
            self.provider = None;
        }
        status.ok()
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            _ = provider.state.unregister();
        }
    }
}

struct ProviderState {
    registered: AtomicBool,
    handle: AtomicU64,
    enabled: AtomicBool,
    level: AtomicU8,
    keyword_any: AtomicU64,
    keyword_all: AtomicU64,
}

impl ProviderState {
    const fn new() -> Self {
        Self {
            registered: AtomicBool::new(false),
            handle: AtomicU64::new(0),
            enabled: AtomicBool::new(false),
            level: AtomicU8::new(0),
            keyword_any: AtomicU64::new(0),
            keyword_all: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    fn enabled(&self, level: Level, keyword: u64) -> bool {
        if self.handle.load(Ordering::Acquire) == 0 {
            return false;
        }

        if !self.enabled.load(Ordering::Acquire) {
            return false;
        }

        let maximum_level = self.level.load(Ordering::Relaxed);
        if maximum_level != 0 && level.0 > maximum_level {
            return false;
        }

        if keyword == 0 {
            return true;
        }

        let any = self.keyword_any.load(Ordering::Relaxed);
        let all = self.keyword_all.load(Ordering::Relaxed);
        keyword & any != 0 && keyword & all == all
    }

    fn register(&'static self, id: &GUID, metadata: &[u8]) -> Result<WIN32_ERROR> {
        if self
            .registered
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(WIN32_ERROR(ERROR_ALREADY_EXISTS).into());
        }

        let mut handle = 0;
        let status = unsafe {
            EventRegister(
                (id as *const GUID).cast(),
                Some(enable_callback),
                self as *const Self as *const c_void,
                &mut handle,
            )
        };

        if status != 0 {
            self.registered.store(false, Ordering::Release);
            return Err(WIN32_ERROR(status).into());
        }

        self.handle.store(handle, Ordering::Release);
        let metadata_status = unsafe {
            EventSetInformation(
                handle,
                EventProviderSetTraits,
                metadata.as_ptr().cast(),
                metadata.len() as u32,
            )
        };
        Ok(WIN32_ERROR(metadata_status))
    }

    fn unregister(&self) -> WIN32_ERROR {
        self.enabled.store(false, Ordering::Release);
        let handle = self.handle.load(Ordering::Acquire);
        let status = if handle == 0 {
            0
        } else {
            unsafe { EventUnregister(handle) }
        };
        if status == 0 {
            self.handle.store(0, Ordering::Release);
            self.registered.store(false, Ordering::Release);
        }
        WIN32_ERROR(status)
    }

    fn write(&self, descriptor: &EventDescriptor, data: &[EventDataDescriptor<'_>]) -> WIN32_ERROR {
        let handle = self.handle.load(Ordering::Acquire);
        if handle == 0 {
            return WIN32_ERROR(0);
        }

        WIN32_ERROR(unsafe {
            EventWriteTransfer(
                handle,
                &descriptor.0,
                core::ptr::null(),
                core::ptr::null(),
                data.len() as u32,
                data.as_ptr().cast(),
            )
        })
    }
}

unsafe extern "system" fn enable_callback(
    _source_id: *const bindings::GUID,
    control_code: u32,
    level: u8,
    keyword_any: u64,
    keyword_all: u64,
    _filter_data: *const EVENT_FILTER_DESCRIPTOR,
    context: *mut c_void,
) {
    if context.is_null() {
        return;
    }

    let state = unsafe { &*context.cast::<ProviderState>() };
    state.enabled.store(false, Ordering::Release);

    if control_code == EVENT_CONTROL_CODE_ENABLE_PROVIDER as u32 {
        state.level.store(level, Ordering::Relaxed);
        state.keyword_any.store(keyword_any, Ordering::Relaxed);
        state.keyword_all.store(keyword_all, Ordering::Relaxed);
        state.enabled.store(true, Ordering::Release);
    } else if control_code == EVENT_CONTROL_CODE_DISABLE_PROVIDER as u32 {
        state.level.store(0, Ordering::Relaxed);
        state.keyword_any.store(0, Ordering::Relaxed);
        state.keyword_all.store(0, Ordering::Relaxed);
    }
}

/// The native ETW event descriptor used by generated event macros.
#[doc(hidden)]
#[repr(transparent)]
pub struct EventDescriptor(EVENT_DESCRIPTOR);

impl EventDescriptor {
    /// Creates a TraceLogging event descriptor.
    #[doc(hidden)]
    pub const fn __new(id: u16, version: u8, level: Level, keyword: u64) -> Self {
        Self(EVENT_DESCRIPTOR {
            Id: id,
            Version: version,
            Channel: 11,
            Level: level.0,
            Opcode: 0,
            Task: 0,
            Keyword: keyword,
        })
    }
}

/// A native ETW data descriptor used by generated event macros.
#[doc(hidden)]
#[repr(transparent)]
pub struct EventDataDescriptor<'a> {
    inner: EVENT_DATA_DESCRIPTOR,
    lifetime: PhantomData<&'a [u8]>,
}

impl<'a> EventDataDescriptor<'a> {
    /// Creates a descriptor for provider metadata.
    #[doc(hidden)]
    pub fn __provider(metadata: &'a [u8]) -> Self {
        Self::__bytes(metadata, 2)
    }

    /// Creates a descriptor for event metadata.
    #[doc(hidden)]
    pub fn __event(metadata: &'a [u8]) -> Self {
        Self::__bytes(metadata, 1)
    }

    /// Creates a descriptor for an encoded field value.
    #[doc(hidden)]
    pub fn __value<T>(value: &'a T) -> Self {
        Self::__new(value as *const T as usize as u64, size_of::<T>() as u32, 0)
    }

    /// Creates a descriptor for a byte slice.
    #[doc(hidden)]
    pub fn __data<T>(value: &'a [T]) -> Self {
        Self::__new(value.as_ptr() as usize as u64, size_of_val(value) as u32, 0)
    }

    fn __bytes(value: &'a [u8], kind: u32) -> Self {
        Self::__new(value.as_ptr() as usize as u64, value.len() as u32, kind)
    }

    fn __new(ptr: u64, size: u32, kind: u32) -> Self {
        Self {
            inner: EVENT_DATA_DESCRIPTOR {
                Ptr: ptr,
                Size: size,
                Anonymous: EVENT_DATA_DESCRIPTOR_0 { Reserved: kind },
            },
            lifetime: PhantomData,
        }
    }
}

use core::mem::{size_of, size_of_val};

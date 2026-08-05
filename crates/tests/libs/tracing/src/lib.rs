#![cfg(windows)]

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;
    use std::ptr::addr_of;
    use std::sync::atomic::{AtomicU32, Ordering};
    use windows::Win32::*;
    use windows::core::{PCWSTR, PWSTR};
    use windows_tracing::{GUID, HRESULT, Level, WIN32_ERROR, define_provider, write_event};

    const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
    const ERROR_MORE_DATA: u32 = 234;
    const KEYWORD: u64 = 0x2;

    define_provider!(
        BASIC_PROVIDER,
        "WindowsTracingBasicTests",
        id(GUID::from_u128(0x8fdd87d4_aa0b_41d2_8e5e_b74f8dcc7fe1))
    );

    define_provider!(
        ROUNDTRIP_PROVIDER,
        "WindowsTracingRoundtripTests",
        id(GUID::from_u128(0xde58512d_05f6_4f24_b5a6_fa3d72038462))
    );

    #[test]
    fn provider_identity_and_disabled_fast_path() {
        assert_eq!(BASIC_PROVIDER.name(), "WindowsTracingBasicTests");
        assert_eq!(
            BASIC_PROVIDER.id(),
            &GUID::from_u128(0x8fdd87d4_aa0b_41d2_8e5e_b74f8dcc7fe1)
        );
        assert!(!BASIC_PROVIDER.enabled(Level::INFORMATIONAL, KEYWORD));

        static EVALUATIONS: AtomicU32 = AtomicU32::new(0);
        let status = write_event!(
            BASIC_PROVIDER,
            "Disabled",
            level(Level::INFORMATIONAL),
            keyword(KEYWORD),
            u32("Value", {
                EVALUATIONS.fetch_add(1, Ordering::Relaxed);
                1
            }),
        );
        assert!(status.is_ok());
        assert_eq!(EVALUATIONS.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn registration_is_exclusive_and_reusable() {
        // SAFETY: Each registration is dropped before this test module unloads.
        let mut registration = unsafe { BASIC_PROVIDER.register() }.unwrap();
        let error = unsafe { BASIC_PROVIDER.register() }.unwrap_err();
        assert_eq!(WIN32_ERROR::from_error(&error), Some(WIN32_ERROR(183)));
        registration.unregister().unwrap();
        registration.unregister().unwrap();

        // SAFETY: The registration is dropped before this test module unloads.
        drop(unsafe { BASIC_PROVIDER.register() }.unwrap());
    }

    #[test]
    fn trace_logging_roundtrip() {
        let path = std::env::temp_dir().join(format!(
            "windows-tracing-{}-{}.etl",
            std::process::id(),
            unique_id()
        ));
        let mut session = TraceSession::start(&path, ROUNDTRIP_PROVIDER.id());

        // SAFETY: The registration is dropped before this test module unloads.
        let registration = unsafe { ROUNDTRIP_PROVIDER.register() }.unwrap();
        assert!(registration.metadata_status().is_ok());
        assert!(ROUNDTRIP_PROVIDER.enabled(Level::INFORMATIONAL, KEYWORD));
        assert!(ROUNDTRIP_PROVIDER.enabled(Level::LOG_ALWAYS, 0));
        assert!(!ROUNDTRIP_PROVIDER.enabled(Level::VERBOSE, KEYWORD));
        assert!(!ROUNDTRIP_PROVIDER.enabled(Level::INFORMATIONAL, 0x4));

        static FILTERED_EVALUATIONS: AtomicU32 = AtomicU32::new(0);
        let status = write_event!(
            ROUNDTRIP_PROVIDER,
            "Filtered",
            level(Level::VERBOSE),
            keyword(KEYWORD),
            u32("Value", {
                FILTERED_EVALUATIONS.fetch_add(1, Ordering::Relaxed);
                1
            }),
        );
        assert!(status.is_ok());
        assert_eq!(FILTERED_EVALUATIONS.load(Ordering::Relaxed), 0);

        let guid = GUID::from_u128(0x00112233_4455_6677_8899_aabbccddeeff);
        let utf16 = [0x03bb, 0x03c9];
        let binary = [0xde, 0xad, 0xbe, 0xef];
        let status = write_event!(
            ROUNDTRIP_PROVIDER,
            "AllFields",
            id_version(42, 3),
            level(Level::INFORMATIONAL),
            keyword(KEYWORD),
            bool("Bool", true),
            i8("I8", -8),
            u8("U8", 8),
            i16("I16", -16),
            u16("U16", 16),
            i32("I32", -32),
            u32("U32", 32),
            i64("I64", -64),
            u64("U64", 64),
            f32("F32", 1.25),
            f64("F64", 2.5),
            guid("Guid", guid),
            hresult("HResult", HRESULT(0x80004005u32 as i32)),
            win32_error("Win32Error", WIN32_ERROR(5)),
            str("Utf8", "hello"),
            utf16("Utf16", &utf16),
            binary("Binary", &binary),
        );
        assert!(status.is_ok(), "{status:?}");

        let oversized = "x".repeat(u16::MAX as usize + 1);
        let status = write_event!(
            ROUNDTRIP_PROVIDER,
            "Oversized",
            level(Level::INFORMATIONAL),
            keyword(KEYWORD),
            str("Value", oversized.as_str()),
        );
        assert_eq!(status, WIN32_ERROR(ERROR_MORE_DATA));

        let status = write_event!(
            ROUNDTRIP_PROVIDER,
            "DefaultKeyword",
            level(Level::INFORMATIONAL),
            u32("Value", 123),
        );
        assert!(status.is_ok(), "{status:?}");

        drop(registration);
        session.stop();

        let event = decode_trace(&path, ROUNDTRIP_PROVIDER.id(), "AllFields");
        let default_keyword = decode_trace(&path, ROUNDTRIP_PROVIDER.id(), "DefaultKeyword");
        std::fs::remove_file(&path).unwrap();

        assert_eq!(event.provider_name, "WindowsTracingRoundtripTests");
        assert_eq!(event.id, 42);
        assert_eq!(event.version, 3);
        assert_eq!(event.level, Level::INFORMATIONAL.0);
        assert_eq!(event.keyword, KEYWORD);

        assert_property(&event, "Bool", 13, 0, &1i32.to_ne_bytes());
        assert_property(&event, "I8", 3, 0, &(-8i8).to_ne_bytes());
        assert_property(&event, "U8", 4, 0, &8u8.to_ne_bytes());
        assert_property(&event, "I16", 5, 0, &(-16i16).to_ne_bytes());
        assert_property(&event, "U16", 6, 0, &16u16.to_ne_bytes());
        assert_property(&event, "I32", 7, 0, &(-32i32).to_ne_bytes());
        assert_property(&event, "U32", 8, 0, &32u32.to_ne_bytes());
        assert_property(&event, "I64", 9, 0, &(-64i64).to_ne_bytes());
        assert_property(&event, "U64", 10, 0, &64u64.to_ne_bytes());
        assert_property(&event, "F32", 11, 0, &1.25f32.to_ne_bytes());
        assert_property(&event, "F64", 12, 0, &2.5f64.to_ne_bytes());
        assert_property(&event, "Guid", 15, 0, as_bytes(&guid));
        assert_property(
            &event,
            "HResult",
            7,
            32,
            &HRESULT(0x80004005u32 as i32).0.to_ne_bytes(),
        );
        assert_property(&event, "Win32Error", 8, 30, &5u32.to_ne_bytes());
        assert_property(&event, "Utf8", 301, 35, b"hello");
        assert_property(&event, "Utf16", 300, 0, as_bytes(&utf16));
        assert_property(&event, "Binary", 14, 0, &binary);
        assert_eq!(default_keyword.keyword, 0);
        assert_property(&default_keyword, "Value", 8, 0, &123u32.to_ne_bytes());
    }

    fn assert_property(
        event: &DecodedEvent,
        name: &str,
        input_type: u16,
        output_type: u16,
        value: &[u8],
    ) {
        let property = &event.properties[name];
        assert_eq!(property.input_type, input_type, "{name}");
        assert_eq!(property.output_type, output_type, "{name}");
        assert_eq!(property.value, value, "{name}");
    }

    fn as_bytes<T>(value: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts((value as *const T).cast(), size_of::<T>()) }
    }

    fn unique_id() -> u32 {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    struct TraceProperties {
        buffer: Box<[EVENT_TRACE_PROPERTIES]>,
    }

    impl TraceProperties {
        fn new(logger_name: &[u16], file_name: &[u16]) -> Self {
            let logger_name_offset = size_of::<EVENT_TRACE_PROPERTIES>();
            let file_name_offset = logger_name_offset + size_of_val(logger_name);
            let buffer_size = file_name_offset + size_of_val(file_name);
            let element_count = buffer_size.div_ceil(size_of::<EVENT_TRACE_PROPERTIES>());
            let mut buffer = Vec::with_capacity(element_count);
            buffer.resize_with(element_count, EVENT_TRACE_PROPERTIES::default);
            let mut value = Self {
                buffer: buffer.into_boxed_slice(),
            };

            unsafe {
                let buffer = value.buffer.as_mut_ptr().cast::<u8>();
                std::ptr::copy_nonoverlapping(
                    logger_name.as_ptr().cast::<u8>(),
                    buffer.add(logger_name_offset),
                    size_of_val(logger_name),
                );
                std::ptr::copy_nonoverlapping(
                    file_name.as_ptr().cast::<u8>(),
                    buffer.add(file_name_offset),
                    size_of_val(file_name),
                );
            }

            let properties = value.properties_mut();
            properties.Wnode.BufferSize = buffer_size.try_into().unwrap();
            properties.Wnode.ClientContext = 1;
            properties.Wnode.Flags = WNODE_FLAG_TRACED_GUID as u32;
            properties.LogFileMode = EVENT_TRACE_FILE_MODE_SEQUENTIAL as u32;
            properties.LoggerNameOffset = logger_name_offset.try_into().unwrap();
            properties.LogFileNameOffset = file_name_offset.try_into().unwrap();
            value
        }

        fn properties_mut(&mut self) -> &mut EVENT_TRACE_PROPERTIES {
            &mut self.buffer[0]
        }
    }

    struct TraceSession {
        handle: CONTROLTRACE_ID,
        logger_name: Vec<u16>,
        properties: TraceProperties,
        stopped: bool,
    }

    impl TraceSession {
        fn start(path: &Path, provider: &GUID) -> Self {
            let logger_name = wide(format!(
                "windows-tracing-tests-{}-{}",
                std::process::id(),
                unique_id()
            ));
            let file_name = wide(path);
            let mut properties = TraceProperties::new(&logger_name, &file_name);
            let mut handle = CONTROLTRACE_ID(0);

            let status = unsafe {
                StartTraceW(
                    &mut handle,
                    PCWSTR(logger_name.as_ptr()),
                    properties.properties_mut(),
                )
            };
            assert_eq!(status, 0, "StartTraceW failed: {status}");

            let status = unsafe {
                EnableTraceEx2(
                    handle,
                    provider,
                    EVENT_CONTROL_CODE_ENABLE_PROVIDER as u32,
                    Level::INFORMATIONAL.0,
                    KEYWORD,
                    0,
                    0,
                    None,
                )
            };
            assert_eq!(status, 0, "EnableTraceEx2 failed: {status}");

            Self {
                handle,
                logger_name,
                properties,
                stopped: false,
            }
        }

        fn stop(&mut self) {
            if self.stopped {
                return;
            }
            let status = unsafe {
                ControlTraceW(
                    self.handle,
                    PCWSTR(self.logger_name.as_ptr()),
                    self.properties.properties_mut(),
                    EVENT_TRACE_CONTROL_STOP as u32,
                )
            };
            assert_eq!(status, 0, "ControlTraceW failed: {status}");
            self.stopped = true;
        }
    }

    impl Drop for TraceSession {
        fn drop(&mut self) {
            if !self.stopped {
                _ = unsafe {
                    ControlTraceW(
                        self.handle,
                        PCWSTR(self.logger_name.as_ptr()),
                        self.properties.properties_mut(),
                        EVENT_TRACE_CONTROL_STOP as u32,
                    )
                };
            }
        }
    }

    #[derive(Default)]
    struct Capture {
        provider: GUID,
        event_name: String,
        event: Option<DecodedEvent>,
        error: Option<String>,
    }

    struct DecodedEvent {
        provider_name: String,
        id: u16,
        version: u8,
        level: u8,
        keyword: u64,
        properties: BTreeMap<String, DecodedProperty>,
    }

    struct DecodedProperty {
        input_type: u16,
        output_type: u16,
        value: Vec<u8>,
    }

    fn decode_trace(path: &Path, provider: &GUID, event_name: &str) -> DecodedEvent {
        let mut capture = Box::new(Capture {
            provider: *provider,
            event_name: event_name.to_owned(),
            event: None,
            error: None,
        });
        let file_name = wide(path);
        let mut logfile = EVENT_TRACE_LOGFILEW {
            LogFileName: PWSTR(file_name.as_ptr() as *mut _),
            Anonymous: EVENT_TRACE_LOGFILEW_0 {
                ProcessTraceMode: PROCESS_TRACE_MODE_EVENT_RECORD as u32,
            },
            Anonymous2: EVENT_TRACE_LOGFILEW_1 {
                EventRecordCallback: Some(event_callback),
            },
            Context: (&mut *capture as *mut Capture).cast(),
            ..Default::default()
        };

        let handle = unsafe { OpenTraceW(&mut logfile) };
        assert_ne!(handle, PROCESSTRACE_HANDLE(INVALID_PROCESSTRACE_HANDLE));
        let status = unsafe { ProcessTrace(&[handle], None, None) };
        assert_eq!(status, 0, "ProcessTrace failed: {status}");
        let status = unsafe { CloseTrace(handle) };
        assert_eq!(status, 0, "CloseTrace failed: {status}");
        if let Some(error) = capture.error {
            panic!("{error}");
        }
        capture.event.unwrap()
    }

    unsafe extern "system" fn event_callback(record: *mut EVENT_RECORD) {
        let record = unsafe { &*record };
        if record.UserContext.is_null() {
            return;
        }
        let capture = unsafe { &mut *record.UserContext.cast::<Capture>() };
        if record.EventHeader.ProviderId != capture.provider {
            return;
        }

        let result = std::panic::catch_unwind(|| {
            if event_name(record) != capture.event_name {
                return None;
            }
            decode_event(record)
        });
        let Ok(event) = result else {
            capture.error = Some("event decoding panicked".to_owned());
            return;
        };
        if let Some(event) = event {
            capture.event = Some(event);
        }
    }

    fn event_name(record: &EVENT_RECORD) -> String {
        let buffer = event_information(record);
        let info = buffer.as_ptr().cast::<TRACE_EVENT_INFO>();
        let offset = unsafe { (*info).Anonymous.EventNameOffset };
        wide_string(buffer.as_ptr(), offset)
    }

    fn decode_event(record: &EVENT_RECORD) -> Option<DecodedEvent> {
        let buffer = event_information(record);
        let info = buffer.as_ptr().cast::<TRACE_EVENT_INFO>();
        let event_name_offset = unsafe { (*info).Anonymous.EventNameOffset };
        if event_name_offset == 0 {
            return None;
        }

        let provider_name = wide_string(buffer.as_ptr(), unsafe { (*info).ProviderNameOffset });
        let count = unsafe { (*info).TopLevelPropertyCount } as usize;
        let properties_ptr: *const EVENT_PROPERTY_INFO =
            unsafe { addr_of!((*info).EventPropertyInfoArray).cast() };
        let mut properties = BTreeMap::new();
        let user_data = unsafe {
            std::slice::from_raw_parts(record.UserData.cast(), record.UserDataLength as usize)
        };
        let mut cursor = 0;
        let mut binary_length = None;

        for index in 0..count {
            let property: &EVENT_PROPERTY_INFO = unsafe { &*properties_ptr.add(index) };
            let name = wide_string(buffer.as_ptr(), property.NameOffset);
            let property_type = unsafe { property.Anonymous.nonStructType };
            if name.ends_with(".Length") {
                let value =
                    read_property(&name, user_data, &mut cursor, property_type.InType, None);
                binary_length = Some(u16::from_ne_bytes(value.try_into().unwrap()) as usize);
                continue;
            }
            let value = read_property(
                &name,
                user_data,
                &mut cursor,
                property_type.InType,
                binary_length.take(),
            );
            properties.insert(
                name,
                DecodedProperty {
                    input_type: property_type.InType,
                    output_type: property_type.OutType,
                    value,
                },
            );
        }

        Some(DecodedEvent {
            provider_name,
            id: record.EventHeader.EventDescriptor.Id,
            version: record.EventHeader.EventDescriptor.Version,
            level: record.EventHeader.EventDescriptor.Level,
            keyword: record.EventHeader.EventDescriptor.Keyword,
            properties,
        })
    }

    fn read_property(
        name: &str,
        data: &[u8],
        cursor: &mut usize,
        input_type: u16,
        supplied_length: Option<usize>,
    ) -> Vec<u8> {
        let length = match supplied_length {
            Some(length) => length,
            None => match input_type {
                3 | 4 => 1,
                5 | 6 => 2,
                7 | 8 | 11 | 13 => 4,
                9 | 10 | 12 => 8,
                15 => 16,
                14 | 22 | 23 | 300 | 301 => {
                    assert!(
                        *cursor + 2 <= data.len(),
                        "{name}: missing count at {cursor}"
                    );
                    let length =
                        u16::from_ne_bytes(data[*cursor..*cursor + 2].try_into().unwrap()) as usize;
                    *cursor += 2;
                    length
                }
                _ => panic!("unsupported decoded input type: {input_type}"),
            },
        };
        let end = *cursor + length;
        assert!(
            end <= data.len(),
            "{name}: {input_type} needs {length} bytes at {cursor}, payload is {data:02x?}",
        );
        let value = data[*cursor..end].to_vec();
        *cursor = end;
        value
    }

    fn event_information(record: &EVENT_RECORD) -> Vec<usize> {
        let mut size = 0;
        let status = unsafe { TdhGetEventInformation(record, None, None, &mut size) };
        assert_eq!(status.0, ERROR_INSUFFICIENT_BUFFER);
        let mut buffer = vec![0usize; (size as usize).div_ceil(size_of::<usize>())];
        let status = unsafe {
            TdhGetEventInformation(record, None, Some(buffer.as_mut_ptr().cast()), &mut size)
        };
        assert_eq!(status.0, 0, "TdhGetEventInformation failed: {}", status.0);
        buffer
    }

    fn wide_string(buffer: *const usize, offset: u32) -> String {
        if offset == 0 {
            return String::new();
        }
        let current = unsafe { buffer.cast::<u8>().add(offset as usize).cast::<u16>() };
        let mut length = 0;
        while unsafe { *current.add(length) } != 0 {
            length += 1;
        }
        String::from_utf16(unsafe { std::slice::from_raw_parts(current, length) }).unwrap()
    }

    fn wide(value: impl AsRef<std::ffi::OsStr>) -> Vec<u16> {
        value.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

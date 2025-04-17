mod Windows {
    mod Foundation {
        mod Metadata {
            class ApiInformation {}
            enum DeprecationType {
                Deprecate = 0,
                Remove = 1,
            }
            enum Platform {
                Windows = 0,
                WindowsPhone = 1,
            }
            enum GCPressureAmount {
                Low = 0,
                Medium = 1,
                High = 2,
            }
            interface IApiInformationStatics {
                fn IsTypePresent();
                fn IsMethodPresent();
                fn IsMethodPresent();
                fn IsEventPresent();
                fn IsPropertyPresent();
                fn IsReadOnlyPropertyPresent();
                fn IsWriteablePropertyPresent();
                fn IsEnumNamedValuePresent();
                fn IsApiContractPresent();
                fn IsApiContractPresent();
            }
            enum ThreadingModel {
                STA = 1,
                MTA = 2,
                Both = 3,
                InvalidThreading = 0,
            }
            enum MarshalingType {
                None = 1,
                Agile = 2,
                Standard = 3,
                InvalidMarshaling = 0,
            }
            enum CompositionType {
                Protected = 1,
                Public = 2,
            }
            enum AttributeTargets {
                All = 4294967295,
                Delegate = 1,
                Enum = 2,
                Event = 4,
                Field = 8,
                Interface = 16,
                Method = 64,
                Parameter = 128,
                Property = 256,
                RuntimeClass = 512,
                Struct = 1024,
                InterfaceImpl = 2048,
                ApiContract = 8192,
            }
            enum FeatureStage {
                AlwaysDisabled = 0,
                DisabledByDefault = 1,
                EnabledByDefault = 2,
                AlwaysEnabled = 3,
            }
        }
        mod Numerics {
            struct Matrix3x2 {
                M11: f32,
                M12: f32,
                M21: f32,
                M22: f32,
                M31: f32,
                M32: f32,
            }
            struct Vector2 {
                X: f32,
                Y: f32,
            }
            struct Vector3 {
                X: f32,
                Y: f32,
                Z: f32,
            }
            struct Quaternion {
                X: f32,
                Y: f32,
                Z: f32,
                W: f32,
            }
            struct Rational {
                Numerator: u32,
                Denominator: u32,
            }
            struct Vector4 {
                X: f32,
                Y: f32,
                Z: f32,
                W: f32,
            }
            struct Matrix4x4 {
                M11: f32,
                M12: f32,
                M13: f32,
                M14: f32,
                M21: f32,
                M22: f32,
                M23: f32,
                M24: f32,
                M31: f32,
                M32: f32,
                M33: f32,
                M34: f32,
                M41: f32,
                M42: f32,
                M43: f32,
                M44: f32,
            }
            struct Plane {
                Normal: Windows::Foundation::Numerics::Vector3,
                D: f32,
            }
        }
        mod Collections {
            interface IObservableMap {
                fn add_MapChanged();
                fn remove_MapChanged();
            }
            interface IIterable {
                fn First();
            }
            interface IPropertySet {}
            interface IVectorChangedEventArgs {
                fn get_CollectionChange();
                fn get_Index();
            }
            interface IMapView {
                fn Lookup();
                fn get_Size();
                fn HasKey();
                fn Split();
            }
            interface IMap {
                fn Lookup();
                fn get_Size();
                fn HasKey();
                fn GetView();
                fn Insert();
                fn Remove();
                fn Clear();
            }
            interface IObservableVector {
                fn add_VectorChanged();
                fn remove_VectorChanged();
            }
            interface IKeyValuePair {
                fn get_Key();
                fn get_Value();
            }
            interface IMapChangedEventArgs {
                fn get_CollectionChange();
                fn get_Key();
            }
            class ValueSet {}
            interface IIterator {
                fn get_Current();
                fn get_HasCurrent();
                fn MoveNext();
                fn GetMany();
            }
            interface IVector {
                fn GetAt();
                fn get_Size();
                fn GetView();
                fn IndexOf();
                fn SetAt();
                fn InsertAt();
                fn RemoveAt();
                fn Append();
                fn RemoveAtEnd();
                fn Clear();
                fn GetMany();
                fn ReplaceAll();
            }
            enum CollectionChange {
                Reset = 0,
                ItemInserted = 1,
                ItemRemoved = 2,
                ItemChanged = 3,
            }
            class PropertySet {}
            interface IVectorView {
                fn GetAt();
                fn get_Size();
                fn IndexOf();
                fn GetMany();
            }
            class StringMap {}
        }
        class GuidHelper {}
        interface IUriRuntimeClass {
            fn get_AbsoluteUri();
            fn get_DisplayUri();
            fn get_Domain();
            fn get_Extension();
            fn get_Fragment();
            fn get_Host();
            fn get_Password();
            fn get_Path();
            fn get_Query();
            fn get_QueryParsed();
            fn get_RawUri();
            fn get_SchemeName();
            fn get_UserName();
            fn get_Port();
            fn get_Suspicious();
            fn Equals();
            fn CombineUri();
        }
        enum AsyncStatus {
            Canceled = 2,
            Completed = 1,
            Error = 3,
            Started = 0,
        }
        class MemoryBuffer {}
        interface IStringable {
            fn ToString();
        }
        class WwwFormUrlDecoderEntry {}
        interface IUriRuntimeClassWithAbsoluteCanonicalUri {
            fn get_AbsoluteCanonicalUri();
            fn get_DisplayIri();
        }
        interface IWwwFormUrlDecoderRuntimeClass {
            fn GetFirstValueByName();
        }
        class PropertyValue {}
        struct EventRegistrationToken {
            Value: i64,
        }
        class WwwFormUrlDecoder {}
        interface IReference {
            fn get_Value();
        }
        struct Rect {
            X: f32,
            Y: f32,
            Width: f32,
            Height: f32,
        }
        struct UniversalApiContract {}
        interface IMemoryBufferReference {
            fn get_Capacity();
            fn add_Closed();
            fn remove_Closed();
        }
        interface IPropertyValueStatics {
            fn CreateEmpty();
            fn CreateUInt8();
            fn CreateInt16();
            fn CreateUInt16();
            fn CreateInt32();
            fn CreateUInt32();
            fn CreateInt64();
            fn CreateUInt64();
            fn CreateSingle();
            fn CreateDouble();
            fn CreateChar16();
            fn CreateBoolean();
            fn CreateString();
            fn CreateInspectable();
            fn CreateGuid();
            fn CreateDateTime();
            fn CreateTimeSpan();
            fn CreatePoint();
            fn CreateSize();
            fn CreateRect();
            fn CreateUInt8Array();
            fn CreateInt16Array();
            fn CreateUInt16Array();
            fn CreateInt32Array();
            fn CreateUInt32Array();
            fn CreateInt64Array();
            fn CreateUInt64Array();
            fn CreateSingleArray();
            fn CreateDoubleArray();
            fn CreateChar16Array();
            fn CreateBooleanArray();
            fn CreateStringArray();
            fn CreateInspectableArray();
            fn CreateGuidArray();
            fn CreateDateTimeArray();
            fn CreateTimeSpanArray();
            fn CreatePointArray();
            fn CreateSizeArray();
            fn CreateRectArray();
        }
        interface IUriEscapeStatics {
            fn UnescapeComponent();
            fn EscapeComponent();
        }
        enum PropertyType {
            Empty = 0,
            UInt8 = 1,
            Int16 = 2,
            UInt16 = 3,
            Int32 = 4,
            UInt32 = 5,
            Int64 = 6,
            UInt64 = 7,
            Single = 8,
            Double = 9,
            Char16 = 10,
            Boolean = 11,
            String = 12,
            Inspectable = 13,
            DateTime = 14,
            TimeSpan = 15,
            Guid = 16,
            Point = 17,
            Size = 18,
            Rect = 19,
            OtherType = 20,
            UInt8Array = 1025,
            Int16Array = 1026,
            UInt16Array = 1027,
            Int32Array = 1028,
            UInt32Array = 1029,
            Int64Array = 1030,
            UInt64Array = 1031,
            SingleArray = 1032,
            DoubleArray = 1033,
            Char16Array = 1034,
            BooleanArray = 1035,
            StringArray = 1036,
            InspectableArray = 1037,
            DateTimeArray = 1038,
            TimeSpanArray = 1039,
            GuidArray = 1040,
            PointArray = 1041,
            SizeArray = 1042,
            RectArray = 1043,
            OtherTypeArray = 1044,
        }
        struct FoundationContract {}
        interface IAsyncInfo {
            fn get_Id();
            fn get_Status();
            fn get_ErrorCode();
            fn Cancel();
            fn Close();
        }
        struct TimeSpan {
            Duration: i64,
        }
        interface IAsyncOperationWithProgress {
            fn put_Progress();
            fn get_Progress();
            fn put_Completed();
            fn get_Completed();
            fn GetResults();
        }
        struct DateTime {
            UniversalTime: i64,
        }
        interface IMemoryBuffer {
            fn CreateReference();
        }
        class Uri {}
        struct Size {
            Width: f32,
            Height: f32,
        }
        interface IUriRuntimeClassFactory {
            fn CreateUri();
            fn CreateWithRelativeUri();
        }
        interface IWwwFormUrlDecoderRuntimeClassFactory {
            fn CreateWwwFormUrlDecoder();
        }
        interface IGuidHelperStatics {
            fn CreateNewGuid();
            fn get_Empty();
            fn Equals();
        }
        interface IAsyncActionWithProgress {
            fn put_Progress();
            fn get_Progress();
            fn put_Completed();
            fn get_Completed();
            fn GetResults();
        }
        interface IDeferral {
            fn Complete();
        }
        interface IPropertyValue {
            fn get_Type();
            fn get_IsNumericScalar();
            fn GetUInt8();
            fn GetInt16();
            fn GetUInt16();
            fn GetInt32();
            fn GetUInt32();
            fn GetInt64();
            fn GetUInt64();
            fn GetSingle();
            fn GetDouble();
            fn GetChar16();
            fn GetBoolean();
            fn GetString();
            fn GetGuid();
            fn GetDateTime();
            fn GetTimeSpan();
            fn GetPoint();
            fn GetSize();
            fn GetRect();
            fn GetUInt8Array();
            fn GetInt16Array();
            fn GetUInt16Array();
            fn GetInt32Array();
            fn GetUInt32Array();
            fn GetInt64Array();
            fn GetUInt64Array();
            fn GetSingleArray();
            fn GetDoubleArray();
            fn GetChar16Array();
            fn GetBooleanArray();
            fn GetStringArray();
            fn GetInspectableArray();
            fn GetGuidArray();
            fn GetDateTimeArray();
            fn GetTimeSpanArray();
            fn GetPointArray();
            fn GetSizeArray();
            fn GetRectArray();
        }
        interface IAsyncOperation {
            fn put_Completed();
            fn get_Completed();
            fn GetResults();
        }
        struct HResult {
            Value: i32,
        }
        interface IAsyncAction {
            fn put_Completed();
            fn get_Completed();
            fn GetResults();
        }
        interface IWwwFormUrlDecoderEntry {
            fn get_Name();
            fn get_Value();
        }
        interface IClosable {
            fn Close();
        }
        interface IGetActivationFactory {
            fn GetActivationFactory();
        }
        interface IMemoryBufferFactory {
            fn Create();
        }
        interface IReferenceArray {
            fn get_Value();
        }
        interface IDeferralFactory {
            fn Create();
        }
        class Deferral {}
        struct Point {
            X: f32,
            Y: f32,
        }
        mod Diagnostics {
            enum CausalitySource {
                Application = 0,
                Library = 1,
                System = 2,
            }
            interface ILoggingSessionFactory {
                fn Create();
            }
            class LoggingSession {}
            interface ILoggingChannelFactory2 {
                fn CreateWithOptions();
                fn CreateWithOptionsAndId();
            }
            interface ILoggingOptions {
                fn get_Keywords();
                fn put_Keywords();
                fn get_Tags();
                fn put_Tags();
                fn get_Task();
                fn put_Task();
                fn get_Opcode();
                fn put_Opcode();
                fn get_ActivityId();
                fn put_ActivityId();
                fn get_RelatedActivityId();
                fn put_RelatedActivityId();
            }
            interface IAsyncCausalityTracerStatics {
                fn TraceOperationCreation();
                fn TraceOperationCompletion();
                fn TraceOperationRelation();
                fn TraceSynchronousWorkStart();
                fn TraceSynchronousWorkCompletion();
                fn add_TracingStatusChanged();
                fn remove_TracingStatusChanged();
            }
            interface ILoggingActivity {
                fn get_Name();
                fn get_Id();
            }
            interface ITracingStatusChangedEventArgs {
                fn get_Enabled();
                fn get_TraceLevel();
            }
            enum LoggingFieldFormat {
                Default = 0,
                Hidden = 1,
                String = 2,
                Boolean = 3,
                Hexadecimal = 4,
                ProcessId = 5,
                ThreadId = 6,
                Port = 7,
                Ipv4Address = 8,
                Ipv6Address = 9,
                SocketAddress = 10,
                Xml = 11,
                Json = 12,
                Win32Error = 13,
                NTStatus = 14,
                HResult = 15,
                FileTime = 16,
                Signed = 17,
                Unsigned = 18,
            }
            enum LoggingOpcode {
                Info = 0,
                Start = 1,
                Stop = 2,
                Reply = 6,
                Resume = 7,
                Suspend = 8,
                Send = 9,
            }
            class LoggingChannel {}
            class ErrorDetails {}
            class LoggingActivity {}
            enum LoggingLevel {
                Verbose = 0,
                Information = 1,
                Warning = 2,
                Error = 3,
                Critical = 4,
            }
            interface IErrorReportingSettings {
                fn SetErrorOptions();
                fn GetErrorOptions();
            }
            enum CausalityTraceLevel {
                Required = 0,
                Important = 1,
                Verbose = 2,
            }
            interface ILoggingChannelOptions {
                fn get_Group();
                fn put_Group();
            }
            class AsyncCausalityTracer {}
            interface ILoggingOptionsFactory {
                fn CreateWithKeywords();
            }
            class LoggingFields {}
            interface ILoggingChannel {
                fn get_Name();
                fn get_Enabled();
                fn get_Level();
                fn LogMessage();
                fn LogMessage();
                fn LogValuePair();
                fn LogValuePair();
                fn add_LoggingEnabled();
                fn remove_LoggingEnabled();
            }
            interface ILoggingSession {
                fn get_Name();
                fn SaveToFileAsync();
                fn AddLoggingChannel();
                fn AddLoggingChannel();
                fn RemoveLoggingChannel();
            }
            interface ILoggingChannelFactory {
                fn Create();
            }
            class LoggingChannelOptions {}
            class TracingStatusChangedEventArgs {}
            interface IErrorDetailsStatics {
                fn CreateFromHResultAsync();
            }
            interface ILogFileGeneratedEventArgs {
                fn get_File();
            }
            enum ErrorOptions {
                None = 0,
                SuppressExceptions = 1,
                ForceExceptions = 2,
                UseSetErrorInfo = 4,
                SuppressSetErrorInfo = 8,
            }
            class FileLoggingSession {}
            interface ILoggingTarget {
                fn IsEnabled();
                fn IsEnabled();
                fn IsEnabled();
                fn LogEvent();
                fn LogEvent();
                fn LogEvent();
                fn LogEvent();
                fn StartActivity();
                fn StartActivity();
                fn StartActivity();
                fn StartActivity();
            }
            class LogFileGeneratedEventArgs {}
            interface ILoggingChannel2 {
                fn get_Id();
            }
            class RuntimeBrokerErrorSettings {}
            interface IFileLoggingSessionFactory {
                fn Create();
            }
            interface ILoggingActivity2 {
                fn get_Channel();
                fn StopActivity();
                fn StopActivity();
                fn StopActivity();
            }
            interface ILoggingActivityFactory {
                fn CreateLoggingActivity();
                fn CreateLoggingActivityWithLevel();
            }
            interface ILoggingChannelOptionsFactory {
                fn Create();
            }
            enum CausalityRelation {
                AssignDelegate = 0,
                Join = 1,
                Choice = 2,
                Cancel = 3,
                Error = 4,
            }
            interface ILoggingFields {
                fn Clear();
                fn BeginStruct();
                fn BeginStruct();
                fn EndStruct();
                fn AddEmpty();
                fn AddEmpty();
                fn AddEmpty();
                fn AddUInt8();
                fn AddUInt8();
                fn AddUInt8();
                fn AddUInt8Array();
                fn AddUInt8Array();
                fn AddUInt8Array();
                fn AddInt16();
                fn AddInt16();
                fn AddInt16();
                fn AddInt16Array();
                fn AddInt16Array();
                fn AddInt16Array();
                fn AddUInt16();
                fn AddUInt16();
                fn AddUInt16();
                fn AddUInt16Array();
                fn AddUInt16Array();
                fn AddUInt16Array();
                fn AddInt32();
                fn AddInt32();
                fn AddInt32();
                fn AddInt32Array();
                fn AddInt32Array();
                fn AddInt32Array();
                fn AddUInt32();
                fn AddUInt32();
                fn AddUInt32();
                fn AddUInt32Array();
                fn AddUInt32Array();
                fn AddUInt32Array();
                fn AddInt64();
                fn AddInt64();
                fn AddInt64();
                fn AddInt64Array();
                fn AddInt64Array();
                fn AddInt64Array();
                fn AddUInt64();
                fn AddUInt64();
                fn AddUInt64();
                fn AddUInt64Array();
                fn AddUInt64Array();
                fn AddUInt64Array();
                fn AddSingle();
                fn AddSingle();
                fn AddSingle();
                fn AddSingleArray();
                fn AddSingleArray();
                fn AddSingleArray();
                fn AddDouble();
                fn AddDouble();
                fn AddDouble();
                fn AddDoubleArray();
                fn AddDoubleArray();
                fn AddDoubleArray();
                fn AddChar16();
                fn AddChar16();
                fn AddChar16();
                fn AddChar16Array();
                fn AddChar16Array();
                fn AddChar16Array();
                fn AddBoolean();
                fn AddBoolean();
                fn AddBoolean();
                fn AddBooleanArray();
                fn AddBooleanArray();
                fn AddBooleanArray();
                fn AddString();
                fn AddString();
                fn AddString();
                fn AddStringArray();
                fn AddStringArray();
                fn AddStringArray();
                fn AddGuid();
                fn AddGuid();
                fn AddGuid();
                fn AddGuidArray();
                fn AddGuidArray();
                fn AddGuidArray();
                fn AddDateTime();
                fn AddDateTime();
                fn AddDateTime();
                fn AddDateTimeArray();
                fn AddDateTimeArray();
                fn AddDateTimeArray();
                fn AddTimeSpan();
                fn AddTimeSpan();
                fn AddTimeSpan();
                fn AddTimeSpanArray();
                fn AddTimeSpanArray();
                fn AddTimeSpanArray();
                fn AddPoint();
                fn AddPoint();
                fn AddPoint();
                fn AddPointArray();
                fn AddPointArray();
                fn AddPointArray();
                fn AddSize();
                fn AddSize();
                fn AddSize();
                fn AddSizeArray();
                fn AddSizeArray();
                fn AddSizeArray();
                fn AddRect();
                fn AddRect();
                fn AddRect();
                fn AddRectArray();
                fn AddRectArray();
                fn AddRectArray();
            }
            class LoggingOptions {}
            interface IFileLoggingSession {
                fn get_Name();
                fn AddLoggingChannel();
                fn AddLoggingChannel();
                fn RemoveLoggingChannel();
                fn CloseAndSaveToFileAsync();
                fn add_LogFileGenerated();
                fn remove_LogFileGenerated();
            }
            interface IErrorDetails {
                fn get_Description();
                fn get_LongDescription();
                fn get_HelpUri();
            }
            enum CausalitySynchronousWork {
                CompletionNotification = 0,
                ProgressNotification = 1,
                Execution = 2,
            }
        }
    }
}

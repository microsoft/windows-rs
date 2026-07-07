// A GUID-less *marker* interface: derives from a COM interface base but declares
// neither a `__declspec(uuid(...))` nor any methods of its own (the SDK's
// `IVssWriterComponentsExt` / `DebugBaseEventCallbacks` idiom). It must still be
// emitted as an interface so references to it resolve — driven by
// `has_interface_base()`, not by a uuid or by owning any methods.
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};

struct __declspec(uuid("10000000-0000-0000-c000-000000000046")) IThing : public IUnknown {
    virtual int Do() = 0;
};

// GUID-less marker: no uuid, no own methods -> `interface IThingExt: IThing {}`.
struct IThingExt : public IThing {
};

// A data struct that *also* inherits an interface base but carries fields: it must
// stay a `struct` with a leading `Base:` field, never collapse to an empty
// interface (the `MFASYNCRESULT`/`RTWQASYNCRESULT` idiom). Guarded by
// `has_data_fields()`.
typedef struct tagThingResult : public IThing {
    void* context;
    int status;
} ThingResult;

struct __declspec(uuid("30000000-0000-0000-c000-000000000046")) IUser : public IUnknown {
    virtual int Use(IThingExt* ext) = 0;
};

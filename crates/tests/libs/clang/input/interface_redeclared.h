// Old-style `DECLARE_INTERFACE_` COM headers (e.g. Direct3D 9, DirectSound,
// XAudio2) redeclare the entire inherited method chain in every derived
// interface. Those redeclarations override the base-class virtuals and reuse
// their existing vtable slots, so the scraper must drop them — the emitted
// `base__` vtable already reconstructs the full inherited chain. Re-emitting
// them would double the inherited slots and corrupt the vtable layout.
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};

struct __declspec(uuid("10000000-0000-0000-c000-000000000046")) IResource : public IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
    virtual int GetType() = 0;
};

struct __declspec(uuid("20000000-0000-0000-c000-000000000046")) ISurface : public IResource {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
    virtual int GetType() = 0;
    virtual int GetDesc(void* desc) = 0;
};

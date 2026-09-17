// Overloaded (same-name) virtual methods must be emitted in reverse declaration
// order within each run, matching MSVC's vtable layout. Non-overloaded methods
// and singleton runs keep their declared order.
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual int QueryInterface(const void* riid, void** ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};

struct IAnimation;

struct __declspec(uuid("10000000-0000-0000-c000-000000000046")) IVisual : public IUnknown {
    // Two-overload run: reversed to (animation, scalar).
    virtual int SetOffsetX(float offsetX) = 0;
    virtual int SetOffsetX(IAnimation* animation) = 0;
    // Non-overloaded method between runs keeps its position.
    virtual int Commit() = 0;
    // Three-overload run: reversed to (pointer, int, float).
    virtual int SetValue(float value) = 0;
    virtual int SetValue(int value) = 0;
    virtual int SetValue(IAnimation* animation) = 0;
};

struct __declspec(uuid("20000000-0000-0000-c000-000000000046")) IAnimation : public IUnknown {
    virtual int GetValue() = 0;
};

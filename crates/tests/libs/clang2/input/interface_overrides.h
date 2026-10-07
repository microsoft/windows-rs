struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IBase {
    virtual int __stdcall F() = 0;
};

struct __declspec(uuid("00000002-0000-0000-c000-000000000046")) IDirect : IBase {
    virtual int __stdcall F() override = 0;
    virtual int __stdcall G() = 0;
};

struct __declspec(uuid("00000003-0000-0000-c000-000000000046")) IMiddle : IBase {
    virtual int __stdcall H() = 0;
};

struct __declspec(uuid("00000004-0000-0000-c000-000000000046")) IIndirect : IMiddle {
    virtual int __stdcall F() override = 0;
    virtual int __stdcall G() = 0;
};

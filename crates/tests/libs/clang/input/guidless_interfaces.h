struct Bare {
    virtual int __stdcall Read(unsigned int value) = 0;
    virtual void __stdcall Reset() = 0;
};

struct Extended : Bare {
    virtual int __stdcall Scale(int value) = 0;
};

struct Inherited : Extended {};

typedef Bare BareAlias;

struct Holder {
    BareAlias* item;
    Inherited* inherited;
    Bare* const* slot;
};

struct ConstHolder {
    const Bare* item;
};

struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) Identified {
    virtual int __stdcall Read() = 0;
};

struct MissingIdentity : Identified {
    virtual void __stdcall Reset() = 0;
};

struct WithData {
    int value;
    virtual void __stdcall Reset() = 0;
};

struct WrongConvention {
    virtual void __cdecl Reset() = 0;
};

#ifndef PURE
#define PURE
#endif

struct Base {
    virtual int __stdcall Read(int value) PURE;
    virtual void __stdcall Reset() PURE;
};

struct Derived : Base {
    virtual int __stdcall Read(int value) PURE;
    virtual int __stdcall Declared(int value);
};

struct Defined {
    virtual int __stdcall Inline(int value) { return value; }
};

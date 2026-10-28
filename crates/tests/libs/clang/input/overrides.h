struct Base {
    virtual int __stdcall Read(unsigned int value) = 0;
    virtual void __stdcall Reset() = 0;
};

struct Derived : Base {
    virtual __declspec(nothrow) void __stdcall Reset() = 0;
    virtual int __stdcall Extra() = 0;
    virtual __declspec(nothrow) int __stdcall Read(unsigned int renamed = 7) = 0;
};

struct Final : Derived {
    virtual __declspec(nothrow) int __stdcall Read(unsigned int final_name = 9) = 0;
    virtual int __stdcall Last() = 0;
};

struct CovariantBase {
    virtual CovariantBase* __stdcall Clone() = 0;
};

struct CovariantChild : CovariantBase {
    virtual CovariantChild* __stdcall Clone() = 0;
};

struct Other {
    virtual int __stdcall Read(unsigned int value) = 0;
};

struct Multiple : Base, Other {
    virtual int __stdcall Read(unsigned int value) = 0;
};

struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IdentifiedBase {
    virtual void __stdcall Read() = 0;
};

struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IdentifiedChild : IdentifiedBase {
    virtual void __stdcall Read() = 0;
};

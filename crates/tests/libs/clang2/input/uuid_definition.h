struct IUnknown;
struct IValue : IUnknown {
    virtual long __stdcall Read(int* value) = 0;
};

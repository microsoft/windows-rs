#define MIDL_INTERFACE(x) struct __declspec(uuid(x)) __declspec(novtable)

MIDL_INTERFACE("6b44d453-343e-4246-826d-208c460655c4") IBase {
    virtual long __stdcall First() = 0;
};

MIDL_INTERFACE("6b44d453-343e-4246-826d-208c460655c5") IDerived : public IBase {
    virtual long __stdcall Second() = 0;
};

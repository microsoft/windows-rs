struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c4")) IFoo {
    virtual long __stdcall Ping() = 0;
};
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c5")) IBar : IFoo {
    virtual long __stdcall Pong() = 0;
};

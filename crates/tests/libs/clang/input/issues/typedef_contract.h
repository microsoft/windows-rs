#pragma once
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c4")) IBase {
    virtual Status __stdcall Ping() = 0;
};
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c5")) IContract : IBase {
    virtual Status __stdcall Reset(Status value, IContract** next) = 0;
};

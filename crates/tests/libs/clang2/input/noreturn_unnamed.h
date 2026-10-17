typedef void (*OrdinaryCallback)(int code);
struct UnnamedStopping {
    void (__attribute__((noreturn)) *stop)(int code);
};
struct Dependent {
    UnnamedStopping value;
};

struct ComputedTrue {
    int (*callback)(int value) noexcept(true);
};
struct ComputedFalse {
    int (*callback)(int value) noexcept(false);
};
struct DynamicNone {
    int (*callback)(int value) throw();
};
struct BareNoexcept {
    int (*callback)(int value) noexcept;
};
struct BareStdcallNoexcept {
    int (__stdcall *callback)(int value) noexcept;
};
struct Safe {
    int value;
};

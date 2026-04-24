// Simulates an API header that intentionally omits its own #include of the
// dependency header; the caller is expected to include the dependency first.
struct A {
    HRESULT h;
};

struct IA {
    virtual void __stdcall A(IA* a) = 0;
};

struct IB {
    virtual void __stdcall A(IA* a, IB* b, IB** result) = 0;
};

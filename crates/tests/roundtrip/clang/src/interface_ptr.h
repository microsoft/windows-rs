struct IA {
    virtual void __stdcall A(IA* a) = 0;
};

struct IB {
    virtual void __stdcall A(IA* a, IB* b, IB** result) = 0;
};

typedef struct IC IC;
typedef struct ID ID;

struct IC {
    virtual void __stdcall Invoke(ID* a, ID** b) = 0;
};

struct ID {
    virtual void __stdcall Invoke(IC* a, IC** b) = 0;
};

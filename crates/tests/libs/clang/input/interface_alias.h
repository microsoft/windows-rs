//! reference interface_alias_ref

struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IExample {
    virtual void Method() = 0;
};

typedef IExample EXAMPLE;
typedef IExample *PEXAMPLE;

void AcceptDirect(EXAMPLE value);
void AcceptPointer(PEXAMPLE value);
EXAMPLE ReturnDirect(void);
PEXAMPLE ReturnPointer(void);
void GetDirect(EXAMPLE **value);
void GetPointer(PEXAMPLE *value);

struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IExternal {
    virtual void Method() = 0;
};

typedef IExternal EXTERNAL;
typedef IExternal *PEXTERNAL;

void AcceptExternal(PEXTERNAL value);
PEXTERNAL ReturnExternal(void);
void GetExternal(PEXTERNAL *value);

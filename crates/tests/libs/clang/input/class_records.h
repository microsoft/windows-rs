#ifndef RECORD
#define RECORD class
#endif
#ifndef VALUE_TYPE
#define VALUE_TYPE int
#endif

RECORD __declspec(uuid("12345678-1234-1234-1234-123456789abc")) Base {
public:
    virtual int __stdcall Read() = 0;
};

RECORD Derived : public Base {
public:
    virtual void __stdcall Reset(int value) = 0;
};

RECORD Inherited : public Derived {};

RECORD Value {
public:
    VALUE_TYPE value;
};

class PrivateField {
    int value;
};

class PrivateMethod {
    virtual void __stdcall Reset() = 0;
};

class PrivateBase : Base {};

class WithData : public Base {
public:
    int value;
};

class Constructed {
public:
    Constructed();
    int value;
};

class Destroyed {
public:
    virtual ~Destroyed();
};

class Empty {};

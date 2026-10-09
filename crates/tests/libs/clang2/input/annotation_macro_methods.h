#define METHODS \
    virtual int __stdcall First(_In_reads_(count) const int* values, int count) = 0; \
    virtual int __stdcall Second(int length, _In_reads_(length) const int* data) = 0;

struct __declspec(uuid("d8e52d74-e90f-4f9f-9817-e19352eaa808")) IMethods {
    METHODS
};

struct __declspec(uuid("27bc8e13-73f9-4f74-a024-78f291fe567d")) IRepeated : IMethods {
    METHODS
};

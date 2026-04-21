// Tests that pointer-to-const fields in structs are emitted as *const T.
// struct.h only exercises *mut T fields.

struct ConstPtrs {
    const int* read_only;
    const unsigned char* bytes;
};

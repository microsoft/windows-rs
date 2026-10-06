//! reference contract_reference_pointer.rdl
struct Base { virtual void Method() = 0; };
typedef Base* Pointer;
typedef Pointer* Handle;
extern "C" void Use(Handle value);

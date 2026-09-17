//! namespace Inheritance
//! no-library
//! args -x c++ --target=x86_64-pc-windows-msvc

typedef struct BASE { int first; } BASE;
             typedef struct OTHER { short third; } OTHER;
             typedef struct DERIVED : BASE, OTHER { void* second; } DERIVED;

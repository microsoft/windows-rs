//! namespace Records
//! no-library
//! args -x c++ -fms-extensions --target=i686-pc-windows-msvc

typedef struct ALIGNED_FIELDS {
                 unsigned int first;
                 void* pointer;
                 __declspec(align(8)) unsigned int count;
                 __declspec(align(8)) unsigned char mode;
                 __declspec(align(8)) void* next;
             } ALIGNED_FIELDS;

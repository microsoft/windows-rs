//! namespace Records
//! no-library
//! args -x c++ --target=i686-pc-windows-msvc

typedef union FLEXIBLE_UNION {
                 int first[];
                 short second[];
             } FLEXIBLE_UNION;

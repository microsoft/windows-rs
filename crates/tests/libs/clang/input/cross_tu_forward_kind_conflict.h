//! namespace ForwardKinds
//! input first.h
typedef struct INNER OUTER;
typedef OUTER* POUTER;
//! input second.h
typedef union INNER { int value; } OUTER;

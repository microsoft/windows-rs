//! namespace Recursive
//! input first.h
typedef struct _LEFT LEFT;
typedef struct _RIGHT RIGHT;
struct _LEFT { RIGHT* right; };
struct _RIGHT { LEFT* left; };
//! input second.h
typedef struct _RIGHT RIGHT;
typedef struct _LEFT LEFT;
struct _RIGHT { LEFT* left; };
struct _LEFT { RIGHT* right; };

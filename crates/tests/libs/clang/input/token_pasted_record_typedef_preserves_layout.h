//! namespace OpaqueKey
//! no-library

#define DECLARE_KEY(name) typedef struct name##__ { long long Internal; } name
             DECLARE_KEY(CONNECTION_KEY);

//! namespace Constants
//! no-library

typedef enum _POOL_TYPE {
                 NonPagedPool = 0,
                 NonPagedPoolCacheAligned = 4,
                 NonPagedPoolNx = 512,
                 NonPagedPoolNxCacheAligned = 516,
             } POOL_TYPE;
             #define NonPagedPool NonPagedPoolNx
             #define NonPagedPoolCacheAligned ((POOL_TYPE)NonPagedPoolNxCacheAligned)

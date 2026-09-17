//! namespace CastAliases
//! no-library

typedef unsigned int REAL_TYPE;
             #define CAST_TYPE REAL_TYPE
             #define CHAIN_TYPE CAST_TYPE
             #define ONE_HOP ((CAST_TYPE)1)
             #define CHAINED ((CHAIN_TYPE)2)
             #define CYCLE_A CYCLE_B
             #define CYCLE_B CYCLE_A
             #define CYCLIC ((CYCLE_A)3)
             #define COLLISION REAL_TYPE
             #undef COLLISION
             #define COLLISION(value) value
             #define FUNCTION_LIKE_COLLISION ((COLLISION)4)

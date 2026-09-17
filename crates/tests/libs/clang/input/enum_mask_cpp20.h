//! args -x c++ -std=c++20

#define TEST_BIT_MASK(n) (~((~0) << n))

typedef enum TEST_VALUES {
    TEST_VALUE = TEST_BIT_MASK(5),
} TEST_VALUES;

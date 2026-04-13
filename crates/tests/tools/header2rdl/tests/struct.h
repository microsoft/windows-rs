typedef struct _POINT {
    int X;
    int Y;
} POINT;

typedef struct _RECT {
    int left;
    int top;
    int right;
    int bottom;
} RECT;

typedef union _LARGE_INTEGER {
    struct {
        unsigned int LowPart;
        int HighPart;
    } u;
    long long QuadPart;
} LARGE_INTEGER;

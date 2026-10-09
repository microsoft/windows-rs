typedef double SECONDS;
typedef enum __MIDL_result {
    ResultNone = 0,
    ResultDone = 1
} RESULT;
typedef struct __MIDL_keyframe {
    int unused;
} *KEYFRAME;
typedef struct __MIDL_point {
    int x;
    int y;
} POINT;

struct Shared { int value; };
typedef Shared FIRST;
typedef Shared SECOND;

struct Values {
    SECONDS time;
    RESULT result;
    KEYFRAME frame;
    POINT point;
    FIRST first;
    SECOND second;
};

extern "C" RESULT Update(SECONDS time, KEYFRAME frame, POINT* point);

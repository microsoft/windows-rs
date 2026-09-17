// Basic struct with primitive fields.
struct Point {
    int x;
    int y;
};

struct Rect {
    struct Point top_left;
    struct Point bottom_right;
};

struct Point {
    int x;
    int y;
};
struct Node;
typedef Node *PNode;
struct Node {
    PNode next;
    int value;
};
union Storage {
    long long wide;
    unsigned char bytes[8];
};
struct Container {
    unsigned char tag;
    Point point;
    Storage storage;
    Node *node;
    const int value;
};
struct Left;
struct Right;
typedef Left *PLeft;
typedef Right *PRight;
struct Left {
    PRight right;
};
struct Right {
    PLeft left;
};
struct Arrays {
    Point points[2];
    volatile int state;
};

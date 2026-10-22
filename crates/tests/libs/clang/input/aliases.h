typedef int Count;
typedef Count OtherCount;
typedef const Count ConstCount;
typedef void Nothing;
typedef Nothing* Handle;
typedef struct Node Node;

struct Node {
    OtherCount value;
    const Node* next;
    ConstCount* constant;
    Handle handle;
};

typedef Count Count;

namespace Shapes {
struct Owner {
    struct Node;
    typedef Node Alias;
    struct Node {
        int value;
        Node *next;
    };
    union Payload {
        int code;
        double value;
    };
    unsigned tag;
    Node nodes[2];
    Alias *link;
    Payload first;
    Payload second;
    struct {
        int left;
        short right;
    } detail;
    union {
        int code;
        struct {
            short low;
            short high;
        };
    };
    union {
        float scalar;
        int integer;
    };
    int Anonymous_7;
};

struct Peer {
    struct Node {
        int value;
        Node *next;
    };
    Node item;
};
}

#pragma pack(push, 2)
struct PackedNested {
    struct Inner {
        char tag;
        int value;
    };
    Inner item;
    union {
        int number;
        short code;
    };
};
#pragma pack(pop)

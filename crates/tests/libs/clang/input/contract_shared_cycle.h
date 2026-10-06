//! file nodes.h
#define NODE(name, first, second) struct name { first* a; second* b; };
//! input first.h
#include "nodes.h"
struct IFoo;
struct Root;
NODE(Leaf, IFoo, Root)
NODE(Node1, Leaf, Leaf)
NODE(Node2, Node1, Leaf)
NODE(Node3, Node2, Node1)
NODE(Node4, Node3, Node2)
NODE(Node5, Node4, Node3)
NODE(Root, Node5, Node4)
//! input second.h
#include "nodes.h"
struct IFoo { virtual void Method() = 0; };
struct Root;
NODE(Leaf, IFoo, Root)
NODE(Node1, Leaf, Leaf)
NODE(Node2, Node1, Leaf)
NODE(Node3, Node2, Node1)
NODE(Node4, Node3, Node2)
NODE(Node5, Node4, Node3)
NODE(Root, Node5, Node4)

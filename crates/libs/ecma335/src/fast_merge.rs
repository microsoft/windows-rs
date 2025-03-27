use super::*;

// TODO: like merge but TypeDef table is sorted by Namespace+Name including nested types
// and NestedClass table is sorted by EnclosingClass rather than NestedClass. This allows
// a single file to be used as a fast in-memory database without first having to index the input.


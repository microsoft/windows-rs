# windows-rdl2

An unpublished authoring proof for a smaller RDL toolchain.

The first checkpoints have a programmatic document model for primitive enums and structs with
named value-type fields and forward references. It emits metadata through `windows-metadata2` and
compares the result with the existing RDL compiler and both metadata readers. Text parsing,
attributes, external references, interfaces, and compatibility with the existing RDL API are
intentionally deferred.

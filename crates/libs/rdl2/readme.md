# windows-rdl2

An unpublished authoring proof for a smaller RDL toolchain.

The first checkpoint has a programmatic document model for primitive enums and structs. It emits
metadata through `windows-metadata2` and validates the result with both metadata readers. Text
parsing, attributes, references, interfaces, and compatibility with the existing RDL API are
intentionally deferred.

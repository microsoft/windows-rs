# windows-rdl2

An unpublished authoring proof for a smaller RDL toolchain.

The first checkpoints have a programmatic document model for primitive enums and structs with
local and external named value-type fields and forward references. It emits metadata through
`windows-metadata2` and compares the result with the existing RDL compiler and both metadata
readers. Source validation reports RDL2 errors with definition and field context, while metadata
construction failures remain separate error sources. Text parsing, attributes, interfaces, and
compatibility with the existing RDL API are intentionally deferred.

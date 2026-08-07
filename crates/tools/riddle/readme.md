# Riddle

Riddle checks, compiles, and formats RDL API descriptions and validates existing metadata.

```text
riddle check api.rdl
riddle build api.rdl --out api.winmd
riddle validate api.winmd
riddle fmt api.rdl
riddle fmt --check api.rdl
```

RDL inputs may be files, directories containing `.rdl` files, or `-` for standard input.
`riddle validate` accepts `.winmd` files and directories. Use `--reference <path>` for additional
metadata references. The default Windows metadata is available for type and attribute resolution
unless `--no-default` is specified.
